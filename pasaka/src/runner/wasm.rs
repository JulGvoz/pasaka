use crate::{
    choice::PassageResult,
    engine::{Engine, EngineState},
    runner::{RenderResult, Runner},
};

pub struct WasmRunner;

#[allow(refining_impl_trait)]
impl Runner for WasmRunner {
    async fn render(
        &mut self,
        engine: &mut Engine,
        prev_text: &[String],
        choice: PassageResult,
    ) -> RenderResult {
        todo!()
    }

    async fn save(&mut self, value: &EngineState) -> bool {
        todo!()
    }

    async fn load(&mut self) -> Option<EngineState> {
        todo!()
    }
}
