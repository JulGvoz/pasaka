use crate::{Passage, PassageImpl, choice::PassageHandle, runner::Runner};

pub struct Engine {
    state: EngineState,
}

struct EngineState {
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
        }
    }

    pub async fn save(&mut self) -> ! {
        todo!()
    }
}
