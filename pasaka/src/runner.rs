use serde::Serialize;

use crate::{
    choice::{ChoiceResult, PassageResult},
    engine::Engine,
};

pub mod cli;

pub trait Runner {
    fn render(
        &mut self,
        engine: &mut Engine,
        prev_text: Vec<String>,
        choice: PassageResult,
    ) -> impl IntoFuture<Output = Option<ChoiceResult>>;

    fn save<T: Serialize>(&mut self, key: &str, value: T) -> impl Future<Output = bool>;

    fn load<T: Serialize>(&mut self, key: &str) -> impl Future<Output = Option<T>>;
}
