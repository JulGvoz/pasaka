use ::serde::{Deserialize, Serialize};
use pasaka::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    count: i32,
}

#[passage]
pub fn StartPoint(mut h: PassageHandle, state: GameState) -> PassageResult {
    h.text(format!("Current count: {}", state.count));

    h.choice()
        .option("Increase", |h, mut state: GameState| {
            state.count += 1;
            h.passage(StartPoint, state)
        })
        .option("Decrease", |h, mut state| {
            state.count -= 1;
            h.passage(StartPoint, state)
        })
        .option("Crash Application", |_, _| panic!("Here's a crash"))
        .build(state)
}

fn main() {
    DefaultRunner::run(StartPoint.with_state(GameState { count: 0 }));
}
