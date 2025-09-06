use std::fs::File;

use console::Term;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

use crate::{
    choice::PassageResult,
    engine::{Engine, EngineState},
};

pub struct CliRunner {
    engine: Engine,
}

impl CliRunner {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn run(&mut self) {
        loop {
            Term::stdout().clear_screen().unwrap();

            self.show_text(self.engine.current());

            if self.make_choice() {
                break;
            }
        }
    }

    pub fn show_text(&self, passage: &PassageResult) {
        for line in &passage.text {
            println!("{line}");
        }
        if !passage.text.is_empty() {
            println!();
        }
    }

    pub fn make_choice(&mut self) -> bool {
        loop {
            if self.engine.current().labels.is_empty() {
                let conf = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Exit game")
                    .default(true)
                    .report(false)
                    .interact_opt()
                    .unwrap();
                match conf {
                    Some(true) => return true,
                    Some(false) => {}
                    None => {
                        if self.show_settings() {
                            return false;
                        }
                    }
                }
                continue;
            }
            let opt = Select::with_theme(&ColorfulTheme::default())
                .default(0)
                .items(&self.engine.current().labels)
                .interact_opt()
                .unwrap();
            match opt {
                Some(index) => {
                    self.engine.update(index);
                    break false;
                }
                None => {
                    if self.show_settings() {
                        return false;
                    }
                }
            }
        }
    }

    pub fn show_settings(&mut self) -> bool {
        let sl_choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Settings...")
            .report(false)
            .item("Save game")
            .item("Load game")
            .interact_opt()
            .unwrap();
        match sl_choice {
            Some(0) => {
                self.save();
            }
            Some(1) => {
                if let Some(state) = self.load() {
                    self.engine.load_state(state);
                    return true;
                }
            }
            _ => {}
        };
        true
    }

    fn save(&self) -> bool {
        let path: String = if let Ok(path) = Input::new()
            .with_prompt("Save to file")
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

        let result = serde_json::to_writer(file, self.engine.state());

        result.is_ok()
    }

    fn load(&mut self) -> Option<EngineState> {
        let path: String = Input::new()
            .with_prompt("Load from file")
            .interact_text()
            .ok()?;
        let file = File::open(path).ok()?;
        serde_json::from_reader(file).ok()?
    }
}
