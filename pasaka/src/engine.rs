use std::cell::RefCell;

use console::Term;
use dialoguer::Select;

use crate::choice::{Choice, ChoiceBuilder, ChoiceHandle, Passage};

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

    pub fn run<S>(passage: Passage<S>, state: S) {
        let mut current: Box<dyn FnOnce() -> Choice> = Box::new(move || passage(state));

        loop {
            Term::stdout().clear_screen().unwrap();

            let choice = current();

            for line in &choice.text {
                println!("{line}");
            }
            println!();

            if choice.labels.is_empty() {
                break;
            }

            let index = Select::new()
                .default(0)
                .items(choice.labels)
                .interact()
                .unwrap();

            let handle = ChoiceHandle { _private: () };
            let result = (choice.action)(index, handle);
            current = result.next_passage;
        }
    }
}
