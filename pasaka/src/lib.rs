use crate::{choice::Choice, engine::Engine};

pub mod choice;
pub mod engine;
pub mod runner;

pub trait Passage<S: 'static> {
    fn run(self, engine: &mut Engine, state: S) -> Choice;
}
