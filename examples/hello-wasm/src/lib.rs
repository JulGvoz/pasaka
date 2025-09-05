use pasaka::{
    choice::{PassageHandle, PassageResult},
    engine::Engine,
    runner::wasm::WasmRunner,
};
use pasaka_macro::passage;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    count: i32,
}

#[cfg(target_family = "wasm")]
mod wasm_workaround {
    unsafe extern "C" {
        pub(super) fn __wasm_call_ctors();
    }
}

#[passage]
pub fn StartPoint(mut h: PassageHandle, state: GameState) -> PassageResult {
    h.text(format!("Current count: {}", state.count));

    h.choice()
        .option("Increase", |mut state: GameState, h| {
            state.count += 1;
            h.passage(StartPoint, state)
        })
        .option("Decrease", |mut state: GameState, h| {
            state.count -= 1;
            h.passage(StartPoint, state)
        })
        .build(state)
}

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(target_family = "wasm")]
    unsafe {
        wasm_workaround::__wasm_call_ctors()
    };

    web_sys::console::log_1(&"Hello, WASM!".into());

    wasm_bindgen_futures::spawn_local(Engine::run(
        StartPoint,
        GameState { count: 0 },
        WasmRunner::new("text", "choices", "save", "load"),
    ));
}
