use serde::{Deserialize, Serialize};

use crate::{
    Passage,
    choice::{PassageHandle, PassageResult},
};

pub struct Engine {
    state: EngineState,
    current: PassageResult,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineState {
    prev_text: Vec<String>,
    passage: Passage,
}

impl EngineState {
    pub fn evaluate(&self) -> PassageResult {
        let handle = PassageHandle {
            text_buffer: self.prev_text.clone(),
        };

        let passage = self.passage.clone();

        passage.run(handle)
    }
}

impl Engine {
    pub fn new(passage: Passage) -> Self {
        let state = EngineState {
            prev_text: Vec::new(),
            passage: passage,
        };
        let current = state.evaluate();
        Engine { state, current }
    }

    pub fn current(&self) -> &PassageResult {
        &self.current
    }

    pub fn update(&mut self, choice_index: usize) -> bool {
        if choice_index >= self.current.labels.len() {
            return false;
        }
        // SAFETY: self.current is written back to before leaving this scope
        // furthermore, self is not used anywhere in the next 4 lines
        let current = unsafe { std::ptr::read(&self.current) };

        let choice = (current.action)(choice_index);

        self.state.prev_text = choice.handle.text_buffer;
        self.state.passage = choice.next_passage;

        // SAFETY: &mut is always safe to write to.
        unsafe { std::ptr::write(&mut self.current, self.state.evaluate()) };

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
