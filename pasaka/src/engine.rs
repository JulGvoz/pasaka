use crate::{
    Passage,
    choice::{Choice, ChoiceBuilder},
    runner::Runner,
};

pub struct Engine {
    text_buffer: Vec<String>,
}

impl Engine {
    pub fn text(&mut self, s: impl ToString) {
        self.text_buffer.push(s.to_string());
    }

    pub(crate) fn take_text(&mut self) -> Vec<String> {
        std::mem::take(&mut self.text_buffer)
    }

    pub fn choice<S>(&'_ mut self) -> ChoiceBuilder<'_, S> {
        ChoiceBuilder {
            engine: self,
            options: Vec::new(),
        }
    }

    pub async fn run<S: 'static>(passage: impl Passage<S>, state: S, mut runner: impl Runner) {
        let mut engine = Engine {
            text_buffer: Vec::new(),
        };
        let mut current: Box<dyn FnOnce(&mut Engine) -> Choice> =
            Box::new(move |engine| passage.run(engine, state));

        loop {
            let choice = current(&mut engine);
            let result = runner.render_choice(&mut engine, choice).await;
            match result {
                Some(result) => {
                    current = result.next_passage;
                    engine.text_buffer.extend(result.handle.text_buffer);
                }
                None => break,
            }
        }
    }

    pub async fn save(&mut self) -> ! {
        todo!()
    }
}
