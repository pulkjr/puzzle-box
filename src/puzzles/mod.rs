pub mod active_toggle_puzzle;
pub mod entry_button_sequence_puzzle;

use crate::{puzzles::active_toggle_puzzle::ActiveTogglePuzzle, HardwareBus};

pub trait Puzzle {
    /// Called each tick to update puzzle state.
    fn update(&mut self, hardware: &mut HardwareBus, now_ms: u64);

    /// Returns true if the puzzle is complete.
    fn is_complete(&self) -> bool;

    /// Returns true if the puzzle is unlocked and can be played.
    fn is_unlocked(&self) -> bool;

    fn unlock(&mut self);
}
pub enum ActivePuzzleTypes {
    TogglePuzzle(ActiveTogglePuzzle),
    //TODO: Add other puzzles
}
