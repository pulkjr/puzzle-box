use embedded_hal::adc::{Channel, OneShot};
use embedded_hal::digital::v2::InputPin;
use rp_pico::hal::adc::{Adc, AdcPin};
use rp_pico::hal::gpio::{
    bank0::*, AnyPin, FunctionNull, FunctionSioInput, FunctionSioOutput, Pin, PullDown, PullNone,
};

/// `HardwareBus` owns all hardware peripherals for the puzzle box.
///
/// It provides centralized ownership of GPIO pins and other devices,
/// ensuring safe access to hardware across puzzles and stages.
pub struct HardwareBus {
    pub internal_led: Pin<Gpio25, FunctionSioOutput, PullDown>,
    pub top_button_a: Button<Gpio0>,
    pub top_button_b: Button<Gpio1>,
    pub top_button_c: Button<Gpio2>,
    pub top_button_d: Button<Gpio3>,
    pub indicator_led: Pin<Gpio4, FunctionSioOutput, PullDown>,
    pub toggle_switches: AdcPin<Pin<Gpio26, FunctionSioInput, PullNone>>, // ADC-capable pin
    pub adc: Adc,                                                         // ADC peripheral
}

impl HardwareBus {
    /// Creates a new `HardwareBus` with the given pins.
    pub fn new(
        internal_led: Pin<Gpio25, FunctionSioOutput, PullDown>,
        top_button_a: Pin<Gpio0, FunctionSioInput, PullDown>,
        top_button_b: Pin<Gpio1, FunctionSioInput, PullDown>,
        top_button_c: Pin<Gpio2, FunctionSioInput, PullDown>,
        top_button_d: Pin<Gpio3, FunctionSioInput, PullDown>,
        indicator_led: Pin<Gpio4, FunctionSioOutput, PullDown>,
        toggle_switches: Pin<Gpio26, FunctionNull, PullNone>,
        adc: Adc,
    ) -> Self {
        let toggle_adc_gpio = toggle_switches.into_floating_input();

        let toggle_adc = AdcPin::new(toggle_adc_gpio).unwrap();

        Self {
            internal_led,
            top_button_a: Button::new(top_button_a, 20),
            top_button_b: Button::new(top_button_b, 20),
            top_button_c: Button::new(top_button_c, 20),
            top_button_d: Button::new(top_button_d, 20),
            indicator_led: indicator_led,
            toggle_switches: toggle_adc,
            adc,
        }
    }
    /// Perform a raw ADC conversion on the given pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - A mutable reference to an `AdcPin` representing an ADC-capable GPIO.
    ///
    /// # Returns
    ///
    /// * `u16` - The raw 12-bit ADC value (0–4095).
    ///
    /// # Notes
    ///
    /// - This method is generic over any ADC-capable pin (GPIO26–28 on the RP2040).
    /// - Returns `0` if the read fails (e.g., hardware error).
    pub fn read_adc_raw<P: AnyPin>(&mut self, pin: &mut AdcPin<P>) -> u16
    where
        P: AnyPin,
        AdcPin<P>: Channel<Adc, ID = u8>,
    {
        self.adc.read(pin).unwrap_or(0)
    }

    /// Reads the current value of the toggle switch connected to GPIO26.
    ///
    /// # Description
    ///
    /// This method performs an analog-to-digital conversion (ADC) on the
    /// `toggle_switches` pin, which is configured as an ADC-capable input.
    /// The raw ADC result is a 12-bit value in the range `0..=4095`.
    ///
    /// The raw value is then normalized into a 4-bit range (`0..=15`),
    /// effectively dividing the full ADC range into 16 discrete "buckets."
    /// This makes it easier to interpret the toggle switch position as
    /// a small integer suitable for puzzle logic or state machines.
    ///
    /// # Returns
    ///
    /// * `u8` — A normalized value between `0` and `15` representing the
    ///   toggle switch position.
    ///
    /// # Notes
    ///
    /// - If the ADC read fails, the raw value defaults to `0`.
    /// - Normalization is done by scaling the raw 12-bit value down to 4 bits:
    ///   `(raw * 16 / 4096)`.
    /// - This method is specific to the `toggle_switches` pin owned by
    ///   `HardwareBus`. For generic ADC reads, use [`read_adc_raw`].
    pub fn read_toggle_switch_value(&mut self) -> u8 {
        let raw = self.adc.read(&mut self.toggle_switches).unwrap_or(0);
        (raw as u32 * 16 / 4096) as u8
    }
}
/// A debounced button wrapper that owns its GPIO pin.
///
/// This struct encapsulates a GPIO input pin and tracks the last time
/// it was pressed, applying a configurable debounce interval to filter
/// out mechanical switch chatter.
///
/// # Type Parameters
///
/// * `P` - The GPIO pin identifier (e.g., `Gpio0`, `Gpio1`).
pub struct Button<P: rp_pico::hal::gpio::PinId> {
    /// The physical GPIO pin configured as an input with pull-down.
    pin: Pin<P, FunctionSioInput, PullDown>,
    /// Timestamp (in ms) of the last accepted press event.
    last_pressed_ms: u64,
    /// Minimum interval (in ms) required between presses to avoid bounce.
    debounce_ms: u64,
}

impl<P: rp_pico::hal::gpio::PinId> Button<P> {
    /// Creates a new debounced button.
    ///
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin configured as a pull-down input.
    /// * `debounce_ms` - The debounce interval in milliseconds.
    ///
    /// # Returns
    ///
    /// A new [`Button`] instance ready to track presses.
    pub fn new(pin: Pin<P, FunctionSioInput, PullDown>, debounce_ms: u64) -> Self {
        Self {
            pin,
            last_pressed_ms: 0,
            debounce_ms,
        }
    }

    /// Determines if the button has been pressed, applying debounce logic.
    ///
    /// # Arguments
    ///
    /// * `now_ms` - The current system time in milliseconds.
    ///
    /// # Returns
    ///
    /// * `true` if the button is currently pressed and the debounce interval
    ///   has elapsed since the last accepted press.
    /// * `false` otherwise.
    ///
    /// # Notes
    ///
    /// - Uses `saturating_sub` to safely handle wraparound of the millisecond counter.
    /// - Prevents false triggers caused by mechanical switch bounce.
    /// - This method should be called repeatedly in your main loop with
    ///   the current time value.
    pub fn is_pressed(&mut self, now_ms: u64) -> bool {
        if self.pin.is_high().unwrap_or(false) {
            if now_ms.saturating_sub(self.last_pressed_ms) >= self.debounce_ms {
                self.last_pressed_ms = now_ms;
                return true;
            }
        }
        false
    }
}
