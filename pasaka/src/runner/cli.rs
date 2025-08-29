use console::Term;
use dialoguer::Select;

use crate::{
    choice::{ChoiceHandle, ChoiceResult},
    runner::Runner,
};

pub struct CliRunner;

impl Runner for CliRunner {
    fn run_step(
        &mut self,
        current: Box<dyn FnOnce() -> crate::choice::Choice>,
    ) -> Option<ChoiceResult> {
        Term::stdout().clear_screen().unwrap();

        let choice = current();

        for line in &choice.text {
            println!("{line}");
        }
        println!();

        if choice.labels.is_empty() {
            return None;
        }

        let index = Select::new()
            .default(0)
            .items(choice.labels)
            .interact()
            .unwrap();

        let handle = ChoiceHandle { _private: () };
        let result = (choice.action)(index, handle);
        Some(result)
    }
}
