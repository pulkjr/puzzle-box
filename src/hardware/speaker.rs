use rp_pico::hal::{
    gpio::{bank0::*, FunctionPwm, Pin, PullDown},
    pwm,
};

use embedded_hal::PwmPin;

/// A simple abstraction for controlling a speaker connected to a PWM pin.
///
/// The `Speaker` struct owns a PWM slice and configures one of its channels
/// to drive a GPIO pin. It provides methods for generating simple tones
/// (like a "ding") by adjusting the duty cycle of the PWM output.
pub struct Speaker {
    /// The PWM slice used to generate audio signals.
    ///
    /// This slice contains the configuration for the PWM counter and
    /// provides access to its channels (e.g., `channel_b`).
    slice: pwm::Slice<pwm::Pwm2, pwm::FreeRunning>,
}

impl Speaker {
    /// Creates a new `Speaker` instance.
    ///
    /// # Arguments
    ///
    /// * `pwm2` - The PWM slice to use for audio output. This slice will be enabled.
    /// * `speaker_pin` - The GPIO pin connected to the speaker, configured for PWM output.
    ///
    /// # Example
    ///
    /// ```rust
    /// let speaker = Speaker::new(pwm2, speaker_pin);
    /// ```
    pub fn new(
        mut pwm2: pwm::Slice<pwm::Pwm2, pwm::FreeRunning>,
        speaker_pin: Pin<Gpio5, FunctionPwm, PullDown>,
    ) -> Self {
        pwm2.enable();
        pwm2.channel_b.output_to(speaker_pin);

        Speaker { slice: pwm2 }
    }

    /// Plays a short "ding" sound by setting the duty cycle briefly.
    ///
    /// This method sets the duty cycle of the PWM channel to 50%,
    /// holds it for a short delay, and then turns the output off.
    ///
    /// # Behavior
    ///
    /// - The duty cycle is set to half of the slice's `TOP` value.
    /// - A small delay is introduced using `cortex_m::asm::delay`.
    /// - The duty cycle is reset to zero to stop the tone.
    ///
    /// # Example
    ///
    /// ```rust
    /// speaker.ding();
    /// ```
    pub fn ding(&mut self) {
        let max = self.slice.get_top();

        self.slice.channel_b.set_duty(max / 2);

        cortex_m::asm::delay(10_000);

        self.slice.channel_b.set_duty(0);
    }
}
