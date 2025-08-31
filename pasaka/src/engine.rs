use serde::{Deserialize, Serialize};

use crate::{Passage, PassageImpl, choice::PassageHandle, runner::Runner};

pub struct Engine {
    state: EngineState,
    request_save: bool,
    request_load: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineState {
    prev_text: Vec<String>,
    passage_with_state: Option<Passage>,
}

impl Engine {
    pub async fn run<P: PassageImpl>(passage: P, state: P::State, mut runner: impl Runner) {
        let mut engine = Engine {
            state: EngineState {
                prev_text: Vec::new(),
                passage_with_state: Some(passage.with_state(state)),
            },
            request_save: false,
            request_load: false,
        };

        loop {
            let handle = PassageHandle {
                text_buffer: Vec::new(),
            };
            let passage = if let Some(passage) = engine.state.passage_with_state.take() {
                passage
            } else {
                break;
            };
            let passage_result = passage.run(handle);
            let prev_text = std::mem::take(&mut engine.state.prev_text);
            let result = runner.render(&mut engine, prev_text, passage_result);
            match result.await {
                Some(result) => {
                    engine.state.prev_text = result.handle.text_buffer;
                    engine.state.passage_with_state = Some(result.next_passage);
                }
                None => break,
            }

            if engine.request_save {
                engine.request_save = false;
                runner.save(&engine.state).await;
            }
            if engine.request_load {
                engine.request_load = false;
                if let Some(state) = runner.load().await {
                    engine.state = state;
                }
            }
        }
    }

    pub fn request_save(&mut self) {
        self.request_save = true;
    }

    pub fn request_load(&mut self) {
        self.request_load = true;
    }
}
