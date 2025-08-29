use std::fmt::Display;

use console::Term;
use dialoguer::Select;

pub mod experiment1;
pub mod experiment2;
pub mod experiment3;
pub mod experiment4;
pub mod experiment5;
pub mod experiment6;

pub fn start() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

pub fn wait() {
    let term = Term::stdout();

    // Wait for a single key press
    term.read_key().unwrap();
}

pub fn show<T: Display>(v: T) {
    print!("{}", v)
}

pub fn showln<T: Display>(v: T) {
    println!("{}", v)
}

pub struct ChoiceOption<T> {
    label: String,
    action: Box<dyn FnOnce() -> T>,
}

impl<T> ChoiceOption<T> {
    pub fn new(label: &str, action: impl FnOnce() -> T + 'static) -> Self {
        Self {
            label: label.to_owned(),
            action: Box::new(action),
        }
    }
}

pub fn choice<T>(choices: Vec<ChoiceOption<T>>) -> T {
    let mut choice_prompts = vec![];
    let mut fns = vec![];

    for choice in choices {
        choice_prompts.push(choice.label);
        fns.push(choice.action);
    }

    let index = Select::new()
        .default(0)
        .items(choice_prompts)
        .interact()
        .unwrap();

    let f = fns.swap_remove(index);
    f()
}
