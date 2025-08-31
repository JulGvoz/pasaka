use std::fmt::Display;

use crate::{
    choice::{ChoiceResult, PassageResult},
    engine::{Engine, EngineState},
};

pub mod cli;

pub enum RenderResult {
    Choice(ChoiceResult),
    Save,
    Load,
    Exit,
}

pub trait Runner {
    fn render(
        &mut self,
        engine: &mut Engine,
        prev_text: &[String],
        choice: PassageResult,
    ) -> impl IntoFuture<Output = RenderResult>;

    fn error(&mut self, error: impl Display) -> impl Future<Output = ()> {
        async move { panic!("{}", error) }
    }

    fn save(&mut self, value: &EngineState) -> impl Future<Output = bool>;

    fn load(&mut self) -> impl Future<Output = Option<EngineState>>;
}
