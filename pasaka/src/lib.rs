use crate::{choice::Choice, engine::Engine};

pub mod choice;
pub mod engine;
pub mod runner;

pub trait Passage<S: 'static>: Clone + serde::Serialize + for<'a> serde::Deserialize<'a>
where
    Self: 'static,
{
    fn run(self, engine: &mut Engine, state: S) -> Choice;

    fn with_state(self, state: S) -> PassageWithState {
        PassageWithState(Box::new(move |engine: &mut Engine| self.run(engine, state)))
    }
}

pub struct PassageWithState(Box<dyn FnOnce(&mut Engine) -> Choice>);
