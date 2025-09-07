//! # Example
//! ```
//! # #[macro_use] extern crate pasaka;
//! use pasaka::*;
//!
//! #[passage]
//! fn Example(mut h: PassageHandle, state: i32) -> PassageResult {
//!     h.text("Text can be outputted using .text.")
//!         .text("It can also be chained.")
//!         .text(format!("Count: {state}"));
//!
//!     h.choice()
//!         .option("The expression is usually a choice", |h, state| {
//!             h.passage(Example, state + 1)
//!         })
//!         .build(state)
//! }
//!
//! # fn main() {}
//! ```

extern crate self as pasaka;

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

mod choice;
pub use choice::PassageHandle;
pub use choice::PassageResult;

mod engine;
pub use engine::Engine;

mod runner;
pub use runner::*;

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

    fn name(&self) -> &'static str;
}

/// Combination of a passage name together with its state.
///
/// [Passage] can be used to provide a "callback".
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
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

/// Register passage to the global registry.
/// You should not call this function,
/// as the [`#[passage]`](passage) macro calls it for you.
pub fn register_passage<P: PassageImpl + Send + Sync>(name: &'static str, passage: P) {
    PASSAGE_REGISTRY
        .lock()
        .expect("registering passage registry should not panic")
        .insert(
            name.to_string(),
            Box::new(move |h, value| {
                let state: P::State = serde_json::from_value(value)
                    .expect("deserialized value should match passage state");
                passage.run(h, state)
            }),
        );
}

pub use pasaka_macro::passage;

/// Re-exports for use inside of the [`#[passage]`](passage) macro.
pub mod macro_support {
    /// Re-export for use inside of the [`#[passage]`](crate::passage) macro.
    pub mod ctor {
        pub use ctor::*;
    }

    /// Re-export for use inside of the [`#[passage]`](crate::passage) macro.
    pub mod serde {
        pub use serde::*;
    }

    /// Re-export for use inside of the [`#[passage]`](crate::passage) macro.
    #[cfg(feature = "web")]
    pub mod yew {
        pub use yew::*;
    }
}
