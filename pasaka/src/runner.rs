use serde::Serialize;

use crate::choice::{Choice, ChoiceResult};

pub mod cli;

pub trait Runner {
    fn render_choice(&mut self, choice: Choice) -> impl IntoFuture<Output = Option<ChoiceResult>>;

    fn save<T: Serialize>(&mut self, key: &str, value: T) -> impl Future<Output = bool>;

    fn load<T: Serialize>(&mut self, key: &str) -> impl Future<Output = Option<T>>;
}
