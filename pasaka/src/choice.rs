use crate::{Passage, PassageWithState};

pub struct PassageResult {
    pub(crate) text: Vec<String>,
    pub(crate) labels: Vec<String>,
    pub(crate) action: Box<dyn FnOnce(usize) -> ChoiceResult>,
}

pub struct PassageHandle {
    pub(crate) text_buffer: Vec<String>,
}

impl PassageHandle {
    pub fn text(&mut self, s: impl ToString) -> &mut Self {
        self.text_buffer.push(s.to_string());
        self
    }

    pub fn choice<S>(self) -> ChoiceBuilder<S> {
        ChoiceBuilder {
            text: self.text_buffer,
            options: Vec::new(),
        }
    }
}

pub(crate) struct ChoiceOption<S> {
    label: String,
    on_choose: Box<dyn FnOnce(S, ChoiceHandle) -> ChoiceResult>,
}

pub struct ChoiceBuilder<S: 'static> {
    text: Vec<String>,
    pub(crate) options: Vec<ChoiceOption<S>>,
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

    pub fn build(mut self, state: S) -> PassageResult {
        let text = self.text;
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

        PassageResult {
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

    pub fn passage<S: 'static>(self, passage: impl Passage<S> + 'static, s: S) -> ChoiceResult {
        self.passage_with_state(passage.with_state(s))
    }

    pub fn passage_with_state(self, passage: PassageWithState) -> ChoiceResult {
        ChoiceResult {
            next_passage: passage,
            handle: self,
        }
    }
}

pub struct ChoiceResult {
    pub(crate) next_passage: PassageWithState,
    pub(crate) handle: ChoiceHandle,
}
