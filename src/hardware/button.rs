use embedded_hal::digital::v2::InputPin;
use rp_pico::hal::gpio::{FunctionSioInput, Pin, PullUp};

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
    /// The physical GPIO pin configured as an input with pull-up.
    pin: Pin<P, FunctionSioInput, PullUp>,
    /// Timestamp (in ms) of the last accepted press event.
    last_pressed_ms: u64,
    /// Minimum interval (in ms) required between presses to avoid bounce.
    debounce_ms: u64,
    // Record if the button was pressed and is still pressed
    was_pressed: bool,
}

impl<P: rp_pico::hal::gpio::PinId> Button<P> {
    /// Creates a new debounced button.
    ///
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin configured as a pull-up input.
    /// * `debounce_ms` - The debounce interval in milliseconds.
    ///
    /// # Returns
    ///
    /// A new [`Button`] instance ready to track presses.
    pub fn new(pin: Pin<P, FunctionSioInput, PullUp>, debounce_ms: u64) -> Self {
        Self {
            pin,
            last_pressed_ms: 0,
            debounce_ms,
            was_pressed: false,
        }
    }

    /// Checks whether the button has just been pressed, with debouncing.
    ///
    /// This method performs **edge detection** rather than simply reporting the
    /// current pin level. It returns `true` only once per physical press event,
    /// even if the button is held down for a long time. Subsequent calls while
    /// the button remains pressed will return `false` until the button is released
    /// and pressed again.
    ///
    /// Debouncing is applied using the configured `debounce_ms` interval to filter
    /// out mechanical noise and rapid toggling.
    ///
    /// # Parameters
    /// - `now_ms`: The current timestamp in milliseconds, typically from a monotonic
    ///   clock source.
    ///
    /// # Returns
    /// - `true` if the button transitioned from released to pressed and the debounce
    ///   interval has elapsed.
    /// - `false` otherwise (button not pressed, still held down, or within debounce).
    ///
    /// # Example
    /// ```
    /// if button.check_pressed(now_ms) {
    ///     println!("Button was pressed!");
    /// }
    /// ```
    pub fn check_pressed(&mut self, now_ms: u64) -> bool {
        let pressed = self.pin.is_low().unwrap_or(false);

        if pressed {
            // Only fire if it was NOT pressed before
            if !self.was_pressed && now_ms.saturating_sub(self.last_pressed_ms) >= self.debounce_ms
            {
                self.was_pressed = true;
                self.last_pressed_ms = now_ms;
                return true;
            }
        } else {
            // Reset when released
            self.was_pressed = false;
        }

        false
    }
}
