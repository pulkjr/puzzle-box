use embedded_hal::digital::v2::OutputPin;

use crate::hardware::HardwareBus;

//Story Setup:
//
// The PuzzleBox whispers a riddle:
//
// “Four guardians stand at the gate. Each holds a number, but only in the right order will the path open. Solve the riddle, then press the guardians in sequence.”
//
// Math Puzzle:
//
// Guardian A: 2 + 3
//
// Guardian B: 12 ÷ 3
//
// Guardian C: 4 × 2
//
// Guardian D: 15 − 7
//
// Solutions:
//
// A = 5
// B = 4
// C = 8
// D = 8
//
// Now the trick: the players must order the guardians by ascending value. That gives the sequence: B (4), A (5), C (8), D (8). If two guardians share the same value (like C and D), the riddle text can hint at which comes first (“the one who multiplies before the one who subtracts”).

pub struct EntryButtonComboPuzzle {
    expected_sequence: [ButtonId; 4],
    current_index: usize,
    is_complete: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ButtonId {
    A,
    B,
    C,
    D,
}

impl EntryButtonComboPuzzle {
    pub fn new() -> Self {
        EntryButtonComboPuzzle {
            // Expected order from the math puzzle: B → A → C → D
            expected_sequence: [ButtonId::B, ButtonId::A, ButtonId::C, ButtonId::D],
            current_index: 0,
            is_complete: false,
        }
    }

    pub fn update(&mut self, hardware: &mut HardwareBus, now_ms: u64) {
        // Check each button; if pressed, compare against expected sequence
        if hardware.top_button_a.check_pressed(now_ms) {
            self.check_press(ButtonId::A, hardware);
        }
        if hardware.top_button_b.check_pressed(now_ms) {
            self.check_press(ButtonId::B, hardware);
        }
        if hardware.top_button_c.check_pressed(now_ms) {
            self.check_press(ButtonId::C, hardware);
        }
        if hardware.top_button_d.check_pressed(now_ms) {
            self.check_press(ButtonId::D, hardware);
        }
    }

    fn check_press(&mut self, pressed: ButtonId, hardware: &mut HardwareBus) {
        if self.is_complete {
            return;
        }

        if pressed == self.expected_sequence[self.current_index] {
            hardware.internal_led.set_high().unwrap();
            self.current_index += 1;
            if self.current_index == self.expected_sequence.len() {
                self.is_complete = true;
                hardware.speaker.ding();
            }
        } else {
            // Wrong button resets the puzzle
            hardware.internal_led.set_low().unwrap();
            self.current_index = 0;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
}
