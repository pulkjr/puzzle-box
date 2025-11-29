#![no_std]
pub mod hardware;
pub mod puzzles;
pub mod stages;

use cortex_m::delay::Delay;
use hardware::HardwareBus;

use stages::*;

use crate::stages::EntryStage;

pub struct PuzzleBox {
    pub current_stage: PuzzleBoxStage,
    pub hardware: HardwareBus,
}

impl PuzzleBox {
    /// Creates a new `PuzzleBox` instance.
    ///
    /// # Arguments
    ///
    /// * `hardware` - The `HardwareBus` that owns all hardware peripherals
    ///   (buttons, displays, sensors) used by the puzzle box.
    ///
    /// # Returns
    ///
    /// A `PuzzleBox` initialized in the `EntryStage` with a default
    /// timer display pin set to GPIO 8.
    ///
    /// # Example
    ///
    /// ```
    /// let hardware = HardwareBus::new(...);
    /// let mut box_game = PuzzleBox::new(hardware);
    /// ```
    pub fn new(hardware: HardwareBus) -> Self {
        let entry_stage = EntryStage::new();

        PuzzleBox {
            current_stage: PuzzleBoxStage::Entry(entry_stage),
            hardware,
        }
    }

    /// Advances the puzzle box state machine by one step.
    ///
    /// This method should be called repeatedly inside the main loop
    /// (super loop). It checks the current stage, updates its puzzles,
    /// and transitions to the next stage when conditions are met.
    ///
    /// # Behavior
    ///
    /// * If the current stage is `Entry`, it calls `stage.update()`.
    ///   - When the entry puzzle is complete, the box transitions
    ///     to the `ActiveStage`.
    ///
    /// * If the current stage is `Active`, similar logic will apply
    ///   until the box is failed or successfully completed
    ///
    /// # Example
    ///
    /// ```
    /// loop {
    ///     puzzle_box.tick();
    /// }
    /// ```
    /// pub fn tick<D: DelayMs<u16>>(&mut self, now_ms: u64, delay: &mut D) {
    pub fn tick(&mut self, now_ms: u64, delay: &mut Delay) {
        match &mut self.current_stage {
            PuzzleBoxStage::Entry(stage) => {
                if stage.update(&mut self.hardware, now_ms, delay) {
                    // Transition to Active stage once Entry puzzle is complete
                    self.current_stage = PuzzleBoxStage::Active(ActiveStage::new());
                }
            }
            PuzzleBoxStage::Active(stage) => {
                // TODO: Add logic for active puzzles
                if stage.update(&mut self.hardware, now_ms) {
                    // Transition to Active stage once Entry puzzle is complete
                    self.current_stage = PuzzleBoxStage::FinalTest;
                }
            }
            _ => {
                // TODO: Implement other stages
            }
        }
    }
}
