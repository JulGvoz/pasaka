use serde::{Deserialize, Serialize};

use crate::{
    Passage,
    choice::{ChoiceResult, PassageHandle, PassageResult},
};

pub struct Engine {
    state: EngineState,
    current: PassageResult,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineState {
    history: Vec<StateEntry>,
    history_index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StateEntry {
    prev_text: Vec<String>,
    passage: Passage,
}

impl EngineState {
    pub fn evaluate(&self) -> PassageResult {
        let entry = &self.history[self.history_index];
        let handle = PassageHandle {
            text_buffer: entry.prev_text.clone(),
        };

        let passage = entry.passage.clone();

        passage.run(handle)
    }

    fn push(&mut self, choice: ChoiceResult) {
        // clear redo
        self.history.truncate(self.history_index + 1);

        let entry = StateEntry {
            prev_text: choice.handle.text_buffer,
            passage: choice.next_passage,
        };
        self.history_index = self.history.len();
        self.history.push(entry);
    }
}

impl Engine {
    pub fn new(passage: Passage) -> Self {
        let entry = StateEntry {
            prev_text: Vec::new(),
            passage: passage,
        };
        let state = EngineState {
            history: vec![entry],
            history_index: 0,
        };
        let current = state.evaluate();
        Engine { state, current }
    }

    pub fn current(&self) -> &PassageResult {
        &self.current
    }

    pub fn update(&mut self, choice_index: usize) -> bool {
        assert!(choice_index < self.current.labels.len());

        replace_with::replace_with_or_abort(&mut self.current, |current| {
            let choice = (current.action)(choice_index);

            self.state.push(choice);

            self.state.evaluate()
        });

        true
    }

    pub fn state(&self) -> &EngineState {
        &self.state
    }

    pub fn load_state(&mut self, state: EngineState) {
        self.state = state;
        self.current = self.state.evaluate();
    }
}
