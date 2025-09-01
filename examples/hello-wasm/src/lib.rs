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

#[passage]
pub fn StartPoint(mut h: PassageHandle, state: GameState) -> PassageResult {
    h.text(format!("Current count: {}", state.count));

    h.choice()
        .option("Increase", |mut state: GameState, h| {
            state.count += 1;
            h.passage(StartPoint, state)
        })
        .option("Decrease", |mut state: GameState, h| {
            state.count += 2;
            h.passage(StartPoint, state)
        })
        .build(state)
}

#[wasm_bindgen]
pub fn start() {
    wasm_bindgen_futures::spawn_local(Engine::run(StartPoint, GameState { count: 0 }, WasmRunner));
}
