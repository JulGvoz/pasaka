use serde::{Deserialize, Serialize};

use crate::{
    Passage, PassageImpl,
    choice::{ChoiceResult, PassageHandle, PassageResult},
};

pub struct Engine {
    state: EngineState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineState {
    prev_text: Vec<String>,
    passage: Passage,
}

impl Engine {
    pub fn new(passage: Passage) -> Self {
        Engine {
            state: EngineState {
                prev_text: Vec::new(),
                passage: passage,
            },
        }
    }

    pub fn step(&self) -> PassageResult {
        let handle = PassageHandle {
            text_buffer: self.state.prev_text.clone(),
        };

        let passage = self.state.passage.clone();

        passage.run(handle)
    }

    pub fn update(&mut self, choice_result: ChoiceResult) {
        self.state.prev_text = choice_result.handle.text_buffer;
        self.state.passage = choice_result.next_passage;
    }

    pub fn state(&self) -> &EngineState {
        &self.state
    }

    pub fn load_state(&mut self, state: EngineState) {
        self.state = state;
    }
}
