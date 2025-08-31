use console::Term;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    choice::{ChoiceResult, PassageResult},
    engine::Engine,
    runner::Runner,
};

pub struct CliRunner;

#[allow(refining_impl_trait)]
impl Runner for CliRunner {
    async fn render(
        &mut self,
        engine: &mut Engine,
        prev_text: Vec<String>,
        choice: PassageResult,
    ) -> Option<ChoiceResult> {
        Term::stdout().clear_screen().unwrap();

        for line in &prev_text {
            println!("{line}");
        }
        if !prev_text.is_empty() {
            println!();
        }

        for line in &choice.text {
            println!("{line}");
        }
        if !choice.text.is_empty() {
            println!();
        }

        if choice.labels.is_empty() {
            return None;
        }

        let index: usize = loop {
            let opt = Select::with_theme(&ColorfulTheme::default())
                .default(0)
                .items(&choice.labels)
                .interact_opt()
                .unwrap();
            match opt {
                Some(index) => break index,
                None => {
                    let sl_choice = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Settings...")
                        .report(false)
                        .item("Save game")
                        .item("Load game")
                        .interact_opt()
                        .unwrap();
                    match sl_choice {
                        Some(0) => todo!("save"),
                        Some(1) => todo!("load"),
                        _ => continue,
                    }
                }
            }
        };

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
