use console::Term;
use dialoguer::Select;

use crate::{
    choice::{Choice, ChoiceResult},
    runner::Runner,
};

pub struct CliRunner;

impl Runner for CliRunner {
    #[allow(refining_impl_trait)]
    async fn render_choice(&mut self, choice: Choice) -> Option<ChoiceResult> {
        Term::stdout().clear_screen().unwrap();

        for line in &choice.text {
            println!("{line}");
        }
        println!();

        if choice.labels.is_empty() {
            return None;
        }

        let index: usize = Select::new()
            .default(0)
            .items(choice.labels)
            .interact()
            .unwrap();

        let result: ChoiceResult = (choice.action)(index);

        Some(result)
    }
}
