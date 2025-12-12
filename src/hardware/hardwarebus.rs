use crate::hardware::{AdcToggleSwitch, Lock, Speaker};

use super::button::Button;
use rp_pico::hal::adc::Adc;
use rp_pico::hal::gpio::FunctionPwm;
use rp_pico::hal::gpio::{
    bank0::*, FunctionNull, FunctionSioInput, FunctionSioOutput, Pin, PullDown, PullNone, PullUp,
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
    pub adc_toggle_switch: AdcToggleSwitch<Gpio26>,
    pub speaker: Speaker, // Gpio5
    pub lock1: Lock<Gpio6>,
    pub lock2: Lock<Gpio7>,
    pub lock3: Lock<Gpio8>,
}

impl HardwareBus {
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
        Self {
            internal_led,
            top_button_a: Button::new(top_button_a, 20),
            top_button_b: Button::new(top_button_b, 20),
            top_button_c: Button::new(top_button_c, 20),
            top_button_d: Button::new(top_button_d, 20),
            indicator_led: indicator_led,
            adc_toggle_switch: AdcToggleSwitch::new(toggle_switches, adc),
            speaker: Speaker::new(pwm2, speaker_pin),
            lock1: Lock::new(lock1),
            lock2: Lock::new(lock2),
            lock3: Lock::new(lock3),
        }
    }
}
