use console::Term;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    choice::{Choice, ChoiceResult},
    runner::Runner,
};

pub struct CliRunner;

#[allow(refining_impl_trait)]
impl Runner for CliRunner {
    async fn render_choice(&mut self, choice: Choice) -> Option<ChoiceResult> {
        Term::stdout().clear_screen().unwrap();

        for line in &choice.text {
            println!("{line}");
        }
        println!();

        if choice.labels.is_empty() {
            return None;
        }

        let index: usize = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(choice.labels)
            .interact()
            .unwrap();

        let result: ChoiceResult = (choice.action)(index);

        Some(result)
    }

    async fn save<T: serde::Serialize>(&mut self, key: &str, value: T) -> bool {
        todo!()
    }

    async fn load<T: serde::Serialize>(&mut self, key: &str) -> Option<T> {
        todo!()
    }
}
