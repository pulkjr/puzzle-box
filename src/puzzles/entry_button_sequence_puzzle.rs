use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;

use crate::hardware::HardwareBus;

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

    /// Updates the puzzle state based on button presses.
    ///
    /// Method is meant to be called from the stage's update method in
    /// the super loop. It check each of the four top buttons (A–D) for
    /// press events using the provided timestamp (`now_ms`) for debounce logic.
    /// If a button press is detected, the input is validated against the expected
    /// sequence by calling [`handle_sequence_input`].
    ///
    /// # Arguments
    /// - `hardware`: Reference to the [`HardwareBus`] containing buttons,
    ///   LEDs, speaker, and locks.
    /// - `now_ms`: Current time in milliseconds, used for button debounce.
    /// - `delay`: Delay provider used for blocking operations (e.g., lock
    ///   pulse).
    pub fn update(&mut self, hardware: &mut HardwareBus, now_ms: u64, delay: &mut Delay) {
        // Check each button; if pressed, compare against expected sequence
        if hardware.top_button_a.check_pressed(now_ms) {
            self.handle_sequence_input(ButtonId::A, hardware, delay);
        }
        if hardware.top_button_b.check_pressed(now_ms) {
            self.handle_sequence_input(ButtonId::B, hardware, delay);
        }
        if hardware.top_button_c.check_pressed(now_ms) {
            self.handle_sequence_input(ButtonId::C, hardware, delay);
        }
        if hardware.top_button_d.check_pressed(now_ms) {
            self.handle_sequence_input(ButtonId::D, hardware, delay);
        }
    }

    /// Handles a single button press and validates it against the expected sequence.
    ///
    /// If the pressed button matches the next expected input:
    /// - The indicator LED is turned on.
    /// - The sequence index is advanced.
    /// - If the sequence is complete:
    ///   - The puzzle is marked as complete.
    ///   - A confirmation sound is played.
    ///   - The lock is unlocked for 200 ms.
    ///
    /// If the pressed button does not match:
    /// - The indicator LED is turned off.
    /// - The sequence progress is reset to the beginning.
    ///
    /// # Arguments
    /// - `pressed`: The button identifier (`A`, `B`, `C`, or `D`) that was pressed.
    /// - `hardware`: Reference to the [`HardwareBus`] for controlling LEDs,
    ///   speaker, and lock.
    /// - `delay`: Delay provider used to time the lock pulse.
    fn handle_sequence_input(
        &mut self,
        pressed: ButtonId,
        hardware: &mut HardwareBus,
        delay: &mut Delay,
    ) {
        if self.is_complete {
            return;
        }

        if pressed == self.expected_sequence[self.current_index] {
            hardware.internal_led.set_high().unwrap();
            self.current_index += 1;
            if self.current_index == self.expected_sequence.len() {
                self.is_complete = true;
                hardware.speaker.ding();
                hardware.lock1.unlock(delay);
            }
        } else {
            // Wrong button resets the puzzle
            hardware.internal_led.set_low().unwrap();
            self.current_index = 0;
        }
    }

    /// Returns whether the puzzle sequence has been successfully completed.
    ///
    /// # Returns
    /// - `true` if the puzzle is complete and the lock has been triggered.
    /// - `false` if the puzzle is still in progress or has been reset.
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
}
