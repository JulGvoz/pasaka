use crate::choice::{Choice, ChoiceResult};

pub mod cli;

pub trait Runner {
    fn run_step(&mut self, current: Box<dyn FnOnce() -> Choice>) -> Option<ChoiceResult>;

    fn run_loop(&mut self, start: Box<dyn FnOnce() -> Choice>) {
        let mut current = start;

        loop {
            let result = if let Some(result) = self.run_step(current) {
                result
            } else {
                break;
            };

            current = result.next_passage;
        }
    }
}
