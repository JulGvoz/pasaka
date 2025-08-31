use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

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

    #[deprecated]
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

fn passage_to_fn<P: PassageImpl + Send + Sync>(p: P) -> BoxedPassage {
    Box::new(move |h, value| {
        let s: P::State =
            serde_json::from_value(value).expect("deserialized value should match passage state");
        p.run(h, s)
    })
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Passage {
    state: serde_json::Value,
    fn_name: String,
}

impl Passage {
    pub fn run(self, h: PassageHandle) -> PassageResult {
        let guard = PASSAGE_REGISTRY
            .lock()
            .expect("accessing passage registry should not panic");
        let f = guard
            .get(&self.fn_name)
            .expect("passage should be registered using #[passage]");

        f(h, self.state)
    }
}

type BoxedPassage = Box<dyn Fn(PassageHandle, serde_json::Value) -> PassageResult + Send + Sync>;

static PASSAGE_REGISTRY: LazyLock<Mutex<HashMap<String, BoxedPassage>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn register_passage<P: PassageImpl + Send + Sync>(name: &'static str, passage: P) {
    PASSAGE_REGISTRY
        .lock()
        .expect("registering passage registry should not panic")
        .insert(name.to_string(), passage_to_fn(passage));
}
