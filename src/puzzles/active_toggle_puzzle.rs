use crate::puzzles::Puzzle;

use crate::HardwareBus;

pub struct ActiveTogglePuzzle {
    is_complete: bool,
    unlocked: bool,
    phase: usize,
    sequence: [u8; 4],
}
impl ActiveTogglePuzzle {
    pub fn new() -> Self {
        ActiveTogglePuzzle {
            is_complete: false,
            unlocked: false,
            phase: 0,
            sequence: [3, 7, 12, 5], // example codes
        }
    }
}
impl Puzzle for ActiveTogglePuzzle {
    fn update(&mut self, hardware: &mut HardwareBus, now_ms: u64) {
        if self.is_complete || !self.unlocked {
            return;
        }
        let value = hardware.read_toggle_switch_value();

        if value == self.sequence[self.phase] {
            // advance phase
            self.phase += 1;

            // TODO: Toggle LED indicator light feedback
            if self.phase >= self.sequence.len() {
                self.is_complete = true;
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.is_complete
    }

    fn is_unlocked(&self) -> bool {
        // could be a flag, or depend on another puzzle’s completion
        self.unlocked
    }
    fn unlock(&mut self) {
        self.unlocked = true;
    }
}
