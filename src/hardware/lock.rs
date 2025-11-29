use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::gpio::{FunctionSioOutput, Pin, PullDown};

/// Represents a solenoid lock controlled by a GPIO pin on the Raspberry Pi Pico.
///
/// This struct abstracts the logic of energizing a solenoid lock for a short
/// duration (e.g., 200 ms) to unlock a puzzle box or cabinet. The lock is
/// connected to a MOSFET driver circuit, and the GPIO pin drives the MOSFET gate.
///
/// # Type Parameters
/// - `P`: The specific GPIO pin identifier (e.g., `rp_pico::hal::gpio::bank0::Gpio15`).
///
/// # Example
/// ```ignore
/// let mut lock = Lock::new(pin);
/// lock.unlock(&mut delay);
/// ```
pub struct Lock<P: rp_pico::hal::gpio::PinId> {
    /// The physical GPIO pin configured as an output with pull-down.
    ///
    /// This pin drives the MOSFET gate. A pull-down ensures the MOSFET
    /// remains off at startup until explicitly driven high.
    pin: Pin<P, FunctionSioOutput, PullDown>,
}

impl<P: rp_pico::hal::gpio::PinId> Lock<P> {
    /// Creates a new `Lock` instance from a configured GPIO pin.
    ///
    /// # Arguments
    /// - `pin`: A GPIO pin already configured as `FunctionSioOutput` with `PullDown`.
    ///
    /// # Returns
    /// A `Lock` instance ready to control the solenoid.
    pub fn new(pin: Pin<P, FunctionSioOutput, PullDown>) -> Self {
        Self { pin }
    }
    /// Unlocks the solenoid by energizing it for a fixed duration (200 ms).
    ///
    /// This method sets the GPIO pin high to turn on the MOSFET, allowing
    /// current to flow through the solenoid. After the delay, the pin is
    /// set low again to stop current flow.
    ///
    /// # Arguments
    /// - `delay`: A delay provider implementing `DelayMs<u16>`, typically
    ///   backed by the Pico’s hardware timer.
    ///
    /// # Behavior
    /// - Energizes the solenoid for 200 ms.
    /// - Ensures the lock is not held open longer than recommended, preventing
    ///   overheating or damage.
    ///
    /// # Panics
    /// - If setting the pin high or low fails, `.unwrap()` will panic.
    pub fn unlock<D: embedded_hal::blocking::delay::DelayMs<u16>>(&mut self, delay: &mut D) {
        self.pin.set_high().unwrap();
        delay.delay_ms(200); // 200 ms pulse
        self.pin.set_low().unwrap();
    }
}
