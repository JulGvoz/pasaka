use pasaka::{
    PassageImpl,
    choice::{PassageHandle, PassageResult},
    engine::Engine,
    runner::wasm::WasmRunner,
};
use pasaka_macro::passage;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// #[cfg(target_family = "wasm")]
// mod wasm_workaround {
//     unsafe extern "C" {
//         pub(super) fn __wasm_call_ctors();
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    count: i32,
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
    // #[cfg(target_family = "wasm")]
    // unsafe {
    //     wasm_workaround::__wasm_call_ctors()
    // };

    web_sys::console::log_1(&"Hello, WASM!".into());

    let engine = Engine::new(StartPoint.with_state(GameState { count: 0 }));
    let runner = Box::new(WasmRunner::new(engine, "text", "choices", "save", "load"));
    // We never plan to drop this
    let runner = Box::leak(runner);

    wasm_bindgen_futures::spawn_local(runner.run());
}
