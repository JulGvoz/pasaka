use crate::choice::{PassageHandle, PassageResult};

pub mod choice;
pub mod engine;
pub mod runner;

pub trait PassageImpl: 'static
where
    Self::State: serde::Serialize + for<'a> serde::Deserialize<'a>,
{
    type State;

    fn run(&self, h: PassageHandle, state: Self::State) -> PassageResult;

    fn with_state(&self, state: Self::State) -> Passage {
        Passage {
            state: serde_json::to_value(state).unwrap(),
            fn_name: self.name().to_string(),
        }
    }

    fn box_clone(&self) -> Box<dyn PassageImpl<State = Self::State>>;

    fn name(&self) -> &'static str;

    fn as_fn(&'_ self) -> Box<dyn Fn(PassageHandle, serde_json::Value) -> PassageResult + '_> {
        Box::new(|h, value| {
            let s: Self::State = serde_json::from_value(value)
                .expect("deserialized value should match passage state");
            self.run(h, s)
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Passage {
    state: serde_json::Value,
    fn_name: String,
}

impl Passage {
    pub fn run(self, h: PassageHandle) -> PassageResult {
        todo!()
    }
}
