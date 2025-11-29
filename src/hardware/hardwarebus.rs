use crate::hardware::{Lock, Speaker};

use super::button::Button;
use embedded_hal::adc::{Channel, OneShot};
use rp_pico::hal::adc::{Adc, AdcPin};
use rp_pico::hal::gpio::FunctionPwm;
use rp_pico::hal::gpio::{
    bank0::*, AnyPin, FunctionNull, FunctionSioInput, FunctionSioOutput, Pin, PullDown, PullNone,
    PullUp,
};
use rp_pico::hal::pwm;

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
    pub speaker: Speaker,                                                 // Gpio5
    pub lock1: Lock<Gpio6>,
    pub lock2: Lock<Gpio7>,
    pub lock3: Lock<Gpio8>,
}

impl HardwareBus {
    /// Creates a new `HardwareBus` with the given pins.
    pub fn new(
        internal_led: Pin<Gpio25, FunctionSioOutput, PullDown>,
        top_button_a: Pin<Gpio0, FunctionSioInput, PullUp>,
        top_button_b: Pin<Gpio1, FunctionSioInput, PullUp>,
        top_button_c: Pin<Gpio2, FunctionSioInput, PullUp>,
        top_button_d: Pin<Gpio3, FunctionSioInput, PullUp>,
        indicator_led: Pin<Gpio4, FunctionSioOutput, PullDown>,
        toggle_switches: Pin<Gpio26, FunctionNull, PullNone>,
        adc: Adc,
        speaker_pin: Pin<Gpio5, FunctionPwm, PullDown>,
        pwm2: pwm::Slice<pwm::Pwm2, pwm::FreeRunning>,
        lock1: Pin<Gpio6, FunctionSioOutput, PullDown>,
        lock2: Pin<Gpio7, FunctionSioOutput, PullDown>,
        lock3: Pin<Gpio8, FunctionSioOutput, PullDown>,
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
            speaker: Speaker::new(pwm2, speaker_pin),
            lock1: Lock::new(lock1),
            lock2: Lock::new(lock2),
            lock3: Lock::new(lock3),
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
