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

pub struct ChoiceBuilder<S: 'static> {
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

    pub fn build(mut self, state: S) -> Choice {
        let text = Engine::take_text();
        let labels = self
            .options
            .iter_mut()
            .map(|o| std::mem::take(&mut o.label))
            .collect();

        let handle = ChoiceHandle { _private: () };
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
    pub _private: (),
}

impl ChoiceHandle {
    pub fn passage<S: 'static>(self, p: Passage<S>, s: S) -> ChoiceResult {
        ChoiceResult {
            next_passage: Box::new(move || p(s)),
        }
    }
}

pub struct ChoiceResult {
    pub next_passage: Box<dyn FnOnce() -> Choice>,
}

pub type Passage<S> = fn(S) -> Choice;
