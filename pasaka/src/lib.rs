use crate::choice::{PassageHandle, PassageResult};

pub mod choice;
pub mod engine;
pub mod runner;

pub trait Passage: Copy + 'static {
    type State;

    fn run(self, h: PassageHandle, state: Self::State) -> PassageResult;

    fn with_state(self, state: Self::State) -> PassageWithState {
        PassageWithState(Box::new(move |h: PassageHandle| self.run(h, state)))
    }
}

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// struct BoundPassage<P, S>
// where
//     P: Passage<S> + 'static,
//     S: 'static,
// {
//     passage: P,
//     state: S,
// }

// #[typetag::serde]
// pub trait PassageErased {}

// impl<P, S> PassageErased for BoundPassage<P, S> where P: serde::Serialize + Passage<S> , S: 'static{}

pub struct PassageWithState(Box<dyn FnOnce(PassageHandle) -> PassageResult>);
