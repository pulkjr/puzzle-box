use crate::puzzles::{
    active_toggle_puzzle::ActiveTogglePuzzle, entry_button_sequence_puzzle::EntryButtonComboPuzzle,
    ActivePuzzleTypes, Puzzle,
};
use crate::HardwareBus;
use heapless::Vec;

pub enum PuzzleBoxStage {
    // Waiting to be started
    Entry(EntryStage),

    // Timer has started and waiting for all puzzles to be completed
    Active(ActiveStage),

    FinalTest,

    Failed,

    Success,
}

pub struct EntryStage {
    puzzle: EntryButtonComboPuzzle,
}

impl EntryStage {
    pub fn new() -> Self {
        EntryStage {
            puzzle: EntryButtonComboPuzzle::new(),
        }
    }

    pub fn update(&mut self, hardware: &mut HardwareBus, now_ms: u64) -> bool {
        self.puzzle.update(hardware, now_ms);
        self.puzzle.is_complete()
    }
}
pub struct ActiveStage {
    puzzles: Vec<ActivePuzzleTypes, 2>,
    is_complete: bool,
}

impl ActiveStage {
    pub fn new() -> Self {
        let mut puzzles = Vec::new();
        let toggle = ActiveTogglePuzzle::new();

        _ = puzzles.push(ActivePuzzleTypes::TogglePuzzle(toggle));

        ActiveStage {
            puzzles,
            is_complete: false,
        }
    }

    pub fn update(&mut self, hardware: &mut HardwareBus, now_ms: u64) -> bool {
        for puzzle in self.puzzles.iter_mut() {
            match puzzle {
                ActivePuzzleTypes::TogglePuzzle(p) => {
                    if p.is_unlocked() && !p.is_complete() {
                        p.update(hardware, now_ms);
                        if p.is_complete() {
                            todo!("Should unlock the next puzzle")
                        }
                    }
                } // TODO: add other puzzle types
            }
        }

        // Stage is complete if all puzzles are complete
        self.is_complete = self.puzzles.iter().all(|p| match p {
            ActivePuzzleTypes::TogglePuzzle(p) => p.is_complete(),
        });

        return self.is_complete;
    }
}
