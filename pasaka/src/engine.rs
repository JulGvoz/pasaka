use std::cell::RefCell;

use console::Term;
use dialoguer::Select;

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

    fn take_text() -> Vec<String> {
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

pub struct Choice {
    text: Vec<String>,
    labels: Vec<String>,
    action: Box<dyn FnOnce(usize, ChoiceHandle) -> ChoiceResult>,
}

struct ChoiceOption<S> {
    label: String,
    on_choose: Box<dyn FnOnce(S, ChoiceHandle) -> ChoiceResult>,
}

pub struct ChoiceBuilder<S: 'static> {
    state: S,
    options: Vec<ChoiceOption<S>>,
}

impl<S> ChoiceBuilder<S> {
    pub fn option(
        mut self,
        label: impl ToString,
        f: impl FnOnce(S, ChoiceHandle) -> ChoiceResult + 'static,
    ) -> Self {
        let option = ChoiceOption {
            label: label.to_string(),
            on_choose: Box::new(f),
        };
        self.options.push(option);

        self
    }

    pub fn build(mut self) -> Choice {
        let text = Engine::take_text();
        let labels = self
            .options
            .iter_mut()
            .map(|o| std::mem::take(&mut o.label))
            .collect();

        let action = Box::new(move |index, handle| {
            let option = self
                .options
                .into_iter()
                .nth(index)
                .expect("selected option should be within bounds of possible options");

            (option.on_choose)(self.state, handle)
        });

        Choice {
            text,
            labels,
            action,
        }
    }
}

pub struct ChoiceHandle {
    _private: (),
}

impl ChoiceHandle {
    pub fn passage<S: 'static>(self, p: Passage<S>, s: S) -> ChoiceResult {
        ChoiceResult {
            next_passage: Box::new(move || p(s)),
        }
    }
}

pub struct ChoiceResult {
    next_passage: Box<dyn FnOnce() -> Choice>,
}

pub type Passage<S> = fn(S) -> Choice;
