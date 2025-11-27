use rp_pico::hal::{
    gpio::{bank0::*, FunctionPwm, Pin, PullDown},
    pwm,
};

use embedded_hal::PwmPin;

pub struct Speaker {
    slice: pwm::Slice<pwm::Pwm2, pwm::FreeRunning>,
}

impl Speaker {
    pub fn new(
        mut pwm2: pwm::Slice<pwm::Pwm2, pwm::FreeRunning>,
        speaker_pin: Pin<Gpio5, FunctionPwm, PullDown>,
    ) -> Self {
        pwm2.enable();
        pwm2.channel_b.output_to(speaker_pin);

        Speaker { slice: pwm2 }
    }

    pub fn ding(&mut self) {
        let max = self.slice.get_top();

        self.slice.channel_b.set_duty(max / 2);

        cortex_m::asm::delay(10_000);

        self.slice.channel_b.set_duty(0);
    }
}
