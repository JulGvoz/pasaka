use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{
    Passage,
    choice::{ChoiceResult, PassageHandle, PassageResult},
};

pub struct Engine {
    state: EngineState,
    current: PassageResult,
    history_limit: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineState {
    history: VecDeque<StateEntry>,
    history_index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StateEntry {
    prev_text: Vec<String>,
    passage: Passage,
}

const ENGINE_HISTORY_LIMIT: usize = 40;

impl EngineState {
    #[must_use]
    #[cfg(test)]
    fn current_entry(&self) -> &StateEntry {
        &self.history[self.history_index]
    }

    #[must_use]
    pub fn evaluate(&self) -> PassageResult {
        let entry = &self.history[self.history_index];
        let handle = PassageHandle {
            text_buffer: entry.prev_text.clone(),
        };

        let passage = entry.passage.clone();

        passage.run(handle)
    }

    #[must_use]
    fn push(&mut self, choice: ChoiceResult, limit: usize) -> PassageResult {
        // clear redo
        self.history.truncate(self.history_index + 1);

        let entry = StateEntry {
            prev_text: choice.handle.text_buffer,
            passage: choice.next_passage,
        };
        self.history_index += 1;
        self.history.push_back(entry);

        while self.history.len() > limit + 1 {
            self.history.pop_front();
            self.history_index -= 1;
        }

        self.evaluate()
    }

    #[must_use]
    pub fn can_undo(&self) -> bool {
        self.history.len() > 1 && self.history_index > 0
    }

    #[must_use]
    pub fn undo(&mut self) -> PassageResult {
        self.history_index = self.history_index.saturating_sub(1);

        self.evaluate()
    }

    #[must_use]
    pub fn can_redo(&self) -> bool {
        self.history.len() > 1 && self.history_index + 1 < self.history.len()
    }

    #[must_use]
    pub fn redo(&mut self) -> PassageResult {
        self.history_index += 1;
        self.history_index = self.history_index.min(self.history.len() - 1);

        self.evaluate()
    }
}

impl Engine {
    /// Create a new [Engine] with the given [Passage].
    pub fn new(passage: Passage) -> Self {
        let entry = StateEntry {
            prev_text: Vec::new(),
            passage: passage,
        };
        let state = EngineState {
            history: VecDeque::from_iter(vec![entry]),
            history_index: 0,
        };
        let current = state.evaluate();
        Engine {
            state,
            current,
            history_limit: ENGINE_HISTORY_LIMIT,
        }
    }

    pub fn current(&self) -> &PassageResult {
        &self.current
    }

    pub fn update(&mut self, choice_index: usize) {
        assert!(choice_index < self.current.labels.len());

        replace_with::replace_with(
            &mut self.current,
            || PassageResult {
                text: vec!["An error has occured".to_string()],
                labels: vec![],
                action: Box::new(|_| todo!()),
            },
            |current| {
                let choice = (current.action)(choice_index);

                self.state.push(choice, self.history_limit)
            },
        );
    }

    pub fn undo(&mut self) {
        self.current = self.state.undo();
    }

    pub fn redo(&mut self) {
        self.current = self.state.redo();
    }

    pub fn state(&self) -> &EngineState {
        &self.state
    }

    pub fn load_state(&mut self, state: EngineState) {
        self.state = state;
        self.current = self.state.evaluate();
    }
}

#[cfg(test)]
mod tests {
    use crate::{engine::ENGINE_HISTORY_LIMIT, *};

    #[passage]
    fn Counter(h: PassageHandle, count: usize) -> PassageResult {
        h.choice()
            .option("", |h, count| h.passage(Counter, count + 1))
            .build(count)
    }

    #[test]
    fn run_example() {
        let mut engine = Engine::new(Counter.with_state(0));
        engine.update(0);
        engine.update(0);
        engine.update(0);
        let state: usize = engine.state.current_entry().passage.state().unwrap();
        assert_eq!(state, 3);
    }

    #[test]
    fn undo() {
        let mut engine = Engine::new(Counter.with_state(0));
        engine.update(0);
        engine.update(0);
        engine.update(0);
        engine.undo();
        let state: usize = engine.state.current_entry().passage.state().unwrap();
        assert_eq!(state, 2);
    }

    #[test]
    fn undo_then_redo() {
        let mut engine = Engine::new(Counter.with_state(0));
        engine.update(0);
        engine.update(0);
        engine.update(0);
        engine.undo();
        engine.undo();
        engine.redo();
        let state: usize = engine.state.current_entry().passage.state().unwrap();
        assert_eq!(state, 2);
    }

    #[test]
    fn undo_max() {
        let mut engine = Engine::new(Counter.with_state(0));
        engine.update(0);
        engine.update(0);
        engine.update(0);
        engine.undo();
        engine.undo();
        engine.undo();
        engine.undo();
        let state: usize = engine.state.current_entry().passage.state().unwrap();
        assert_eq!(state, 0);
    }

    #[test]
    fn undo_then_redo_max() {
        let mut engine = Engine::new(Counter.with_state(0));
        engine.update(0);
        engine.update(0);
        engine.update(0);
        engine.undo();
        engine.undo();
        engine.redo();
        engine.redo();
        engine.redo();
        engine.redo();
        let state: usize = engine.state.current_entry().passage.state().unwrap();
        assert_eq!(state, 3);
    }

    #[test]
    fn undo_limit() {
        let mut engine = Engine::new(Counter.with_state(0));

        for _ in 0..200 {
            engine.update(0);
        }
        for _ in 0..200 {
            engine.undo();
        }

        let state: usize = engine.state.current_entry().passage.state().unwrap();
        assert_eq!(state, 200 - ENGINE_HISTORY_LIMIT);
    }
}
