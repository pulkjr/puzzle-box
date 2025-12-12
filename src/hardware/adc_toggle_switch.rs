use embedded_hal::adc::{Channel, OneShot};
use rp_pico::hal::adc::{Adc, AdcPin};
use rp_pico::hal::gpio::{
    FunctionNull, FunctionSio, FunctionSioInput, Pin, PinId, PullNone, SioInput, ValidFunction,
};

pub struct AdcToggleSwitch<P>
where
    AdcPin<Pin<P, FunctionSioInput, PullNone>>: Channel<Adc, ID = u8>,
    P: PinId + ValidFunction<FunctionSio<SioInput>>,
{
    pub pin: AdcPin<Pin<P, FunctionSioInput, PullNone>>, // ADC-capable pin
    pub adc: Adc,                                        // ADC peripheral
}

impl<P> AdcToggleSwitch<P>
where
    P: PinId + ValidFunction<FunctionSio<SioInput>>,
    AdcPin<Pin<P, FunctionSioInput, PullNone>>: Channel<Adc, ID = u8>,
{
    pub fn new(pin: Pin<P, FunctionNull, PullNone>, adc: Adc) -> Self {
        let toggle_adc_gpio = pin.into_floating_input();

        let toggle_adc = AdcPin::new(toggle_adc_gpio).unwrap();

        AdcToggleSwitch {
            pin: toggle_adc,
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
    pub fn read_adc_raw(&mut self) -> u16 {
        self.adc.read(&mut self.pin).unwrap_or(0)
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
        let raw = self.read_adc_raw();
        (raw as u32 * 16 / 4096) as u8
    }
}
