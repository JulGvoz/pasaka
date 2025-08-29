use std::cell::RefCell;

use crate::{
    choice::{Choice, ChoiceBuilder, Passage},
    runner::Runner,
};

thread_local! {
    static TEXT_BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

pub struct Engine {
    _private: (),
}

impl Engine {
    pub fn text(s: impl ToString) {
        // println!("{}", s.to_string())
        TEXT_BUF.with_borrow_mut(|buf| {
            buf.push(s.to_string());
        })
    }

    pub(crate) fn take_text() -> Vec<String> {
        TEXT_BUF.with_borrow_mut(|buf| std::mem::take(buf))
    }

    pub fn choice<S>(state: S) -> ChoiceBuilder<S> {
        ChoiceBuilder {
            state,
            options: Vec::new(),
        }
    }

    pub fn run<S: 'static>(passage: Passage<S>, state: S, mut runner: impl Runner) {
        let current: Box<dyn FnOnce() -> Choice> = Box::new(move || passage(state));

        runner.run_loop(current);
    }
}
