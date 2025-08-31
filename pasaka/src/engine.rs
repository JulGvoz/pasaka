use serde::{Deserialize, Serialize};

use crate::{
    Passage, PassageImpl,
    choice::PassageHandle,
    runner::{RenderResult, Runner},
};

pub struct Engine {
    state: EngineState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EngineState {
    prev_text: Vec<String>,
    passage: Option<Passage>,
}

impl Engine {
    pub async fn run<P: PassageImpl>(passage: P, state: P::State, mut runner: impl Runner) {
        let mut engine = Engine {
            state: EngineState {
                prev_text: Vec::new(),
                passage: Some(passage.with_state(state)),
            },
        };

        loop {
            let handle = PassageHandle {
                text_buffer: Vec::new(),
            };

            let passage = engine
                .state
                .passage
                .take()
                .expect("state should hold current passage");
            let old_passage = passage.clone();
            let passage_result = passage.run(handle);
            let prev_text = std::mem::take(&mut engine.state.prev_text);
            let result = runner.render(&mut engine, &prev_text, passage_result);
            match result.await {
                RenderResult::Choice(choice_result) => {
                    engine.state.prev_text = choice_result.handle.text_buffer;
                    engine.state.passage = Some(choice_result.next_passage);
                }
                RenderResult::Save => {
                    engine.state.prev_text = prev_text;
                    engine.state.passage = Some(old_passage);
                    runner.save(&engine.state).await;
                }
                RenderResult::Load => {
                    if let Some(state) = runner.load().await {
                        engine.state = state;
                    } else {
                        engine.state.passage = Some(old_passage);
                        runner.error("Failed to load").await;
                    }
                }
                RenderResult::Exit => break,
            }
        }
    }
}
