use crate::{
    choice::{ChoiceResult, PassageResult},
    engine::{Engine, EngineState},
};

pub mod cli;

pub trait Runner {
    fn render(
        &mut self,
        engine: &mut Engine,
        prev_text: Vec<String>,
        choice: PassageResult,
    ) -> impl IntoFuture<Output = Option<ChoiceResult>>;

    fn save(&mut self, value: &EngineState) -> impl Future<Output = bool>;

    fn load(&mut self) -> impl Future<Output = Option<EngineState>>;
}
