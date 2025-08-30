use crate::engine::Engine;

pub struct Choice {
    pub(crate) text: Vec<String>,
    pub(crate) labels: Vec<String>,
    pub(crate) action: Box<dyn FnOnce(usize) -> ChoiceResult>,
}

pub(crate) struct ChoiceOption<S> {
    label: String,
    on_choose: Box<dyn FnOnce(S, ChoiceHandle) -> ChoiceResult>,
}

pub struct ChoiceBuilder<'a, S: 'static> {
    pub(crate) engine: &'a mut Engine,
    pub(crate) options: Vec<ChoiceOption<S>>,
}

impl<'a, S> ChoiceBuilder<'a, S> {
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

    pub fn build(mut self, state: S) -> Choice {
        let text = self.engine.take_text();
        let labels = self
            .options
            .iter_mut()
            .map(|o| std::mem::take(&mut o.label))
            .collect();

        let handle = ChoiceHandle {
            text_buffer: Vec::new(),
        };
        let action = Box::new(move |index| {
            let option = self
                .options
                .into_iter()
                .nth(index)
                .expect("selected option should be within bounds of possible options");

            (option.on_choose)(state, handle)
        });

        Choice {
            text,
            labels,
            action,
        }
    }
}

pub struct ChoiceHandle {
    pub(crate) text_buffer: Vec<String>,
}

impl ChoiceHandle {
    pub fn text(&mut self, s: impl ToString) {
        self.text_buffer.push(s.to_string());
    }

    pub fn passage<S: 'static>(self, p: Passage<S>, s: S) -> ChoiceResult {
        ChoiceResult {
            next_passage: Box::new(move |engine| p(engine, s)),
            handle: self,
        }
    }
}

pub struct ChoiceResult {
    pub(crate) next_passage: Box<dyn FnOnce(&mut Engine) -> Choice>,
    pub(crate) handle: ChoiceHandle,
}

pub type Passage<S> = fn(&mut Engine, S) -> Choice;
