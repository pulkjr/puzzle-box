use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use rp_pico::hal::gpio::{FunctionSioOutput, Pin, PullDown};

pub enum StatusLedState {
    Off,
    On,
    Blinking { period_ms: u32 },
}

pub struct StatusLed<P: rp_pico::hal::gpio::PinId> {
    pin: Pin<P, FunctionSioOutput, PullDown>,
    state: StatusLedState,
    timer: u32, // ms accumulator for blinking
}

impl<P: rp_pico::hal::gpio::PinId> StatusLed<P> {
    pub fn new(pin: Pin<P, FunctionSioOutput, PullDown>) -> Self {
        Self {
            pin,
            state: StatusLedState::Off,
            timer: 0,
        }
    }

    pub fn set_state(&mut self, state: StatusLedState) {
        self.state = state;
        if let StatusLedState::On = self.state {
            self.pin.set_high().unwrap();
        } else if let StatusLedState::Off = self.state {
            self.pin.set_low().unwrap();
        }
    }

    /// Call this from your main loop with elapsed milliseconds.
    pub fn update(&mut self, elapsed_ms: u32) {
        match self.state {
            StatusLedState::Blinking { period_ms } => {
                self.timer += elapsed_ms;
                if self.timer >= period_ms {
                    self.timer = 0;
                    self.pin.toggle().unwrap();
                }
            }
            _ => {}
        }
    }
}
