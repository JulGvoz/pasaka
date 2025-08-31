use std::fs::File;

use console::Term;
use dialoguer::{Input, Select, theme::ColorfulTheme};

use crate::{
    choice::{ChoiceResult, PassageResult},
    engine::{Engine, EngineState},
    runner::{RenderResult, Runner},
};

pub struct CliRunner;

#[allow(refining_impl_trait)]
impl Runner for CliRunner {
    async fn render(
        &mut self,
        _engine: &mut Engine,
        prev_text: &[String],
        choice: PassageResult,
    ) -> RenderResult {
        Term::stdout().clear_screen().unwrap();

        for line in prev_text {
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
            return RenderResult::Exit;
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
                        Some(0) => return RenderResult::Save,
                        Some(1) => return RenderResult::Load,
                        _ => continue,
                    }
                }
            }
        };

        let result: ChoiceResult = (choice.action)(index);

        RenderResult::Choice(result)
    }

    async fn save(&mut self, value: &EngineState) -> bool {
        let path: String = if let Ok(path) = Input::new()
            .with_prompt("Save to file:")
            .report(true)
            .interact_text()
        {
            path
        } else {
            return false;
        };
        let file = if let Ok(file) = File::create(path) {
            file
        } else {
            return false;
        };

        let result = serde_json::to_writer(file, value);

        result.is_ok()
    }

    async fn load(&mut self) -> Option<EngineState> {
        let path: String = Input::new()
            .with_prompt("Load file:")
            .interact_text()
            .ok()?;
        let file = File::open(path).ok()?;
        serde_json::from_reader(file).ok()?
    }
}
