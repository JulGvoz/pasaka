use pasaka::{
    PassageImpl,
    choice::{PassageHandle, PassageResult},
    passage,
    runner::web::WebRunner,
};
use serde::{Deserialize, Serialize};

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
        .option("Decrease", |mut state, h| {
            state.count -= 1;
            h.passage(StartPoint, state)
        })
        .build(state)
}

fn main() {
    WebRunner::run(StartPoint.with_state(GameState { count: 0 }));
}
