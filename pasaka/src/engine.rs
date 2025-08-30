use crate::{
    Passage, PassageWithState,
    choice::{PassageHandle, PassageResult},
    runner::Runner,
};

pub struct Engine {
    _private: (),
}

struct EngineState {
    prev_text: Vec<String>,
    passage_with_state: PassageWithState,
}

impl Engine {
    pub async fn run<S: 'static>(passage: impl Passage<S>, state: S, mut runner: impl Runner) {
        let mut engine_state = EngineState {
            prev_text: Vec::new(),
            passage_with_state: passage.with_state(state),
        };

        let mut engine = Engine { _private: () };

        loop {
            let handle = PassageHandle {
                text_buffer: Vec::new(),
            };
            let passage_result = engine_state.passage_with_state.0(handle);
            let result = runner.render(&mut engine, engine_state.prev_text, passage_result);
            match result.await {
                Some(result) => {
                    engine_state.prev_text = result.handle.text_buffer;
                    engine_state.passage_with_state = result.next_passage;
                }
                None => break,
            }
        }
    }

    pub async fn save(&mut self) -> ! {
        todo!()
    }
}
