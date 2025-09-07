use ::serde::{Deserialize, Serialize};
use pasaka::*;

#[derive(Serialize, Deserialize)]
struct State {
    gold: i32,
    left_gold: i32,
}

#[passage]
fn MainHall(mut h: PassageHandle, state: State) -> PassageResult {
    h.text(format!("You have {} gold.", state.gold));

    h.choice()
        .option("Go left", |state, h| h.passage(Left, state))
        .option("Go right", |_, h| h.passage(Right, ()))
        .build(state)
}

#[passage]
fn Left(mut h: PassageHandle, mut state: State) -> PassageResult {
    if state.left_gold > 0 {
        h.text(format!("You find {} gold!", state.left_gold));
        state.gold += state.left_gold;
        state.left_gold = 0;
    } else {
        h.text("The room is empty now...");
    }

    h.choice()
        .option("Return", |state, h| h.passage(MainHall, state))
        .build(state)
}

#[passage]
fn Right(mut h: PassageHandle, _: ()) -> PassageResult {
    h.text("Rocks fall on you, and you die.");

    h.choice().build(())
}

pub fn main() {
    CliRunner::run(MainHall.with_state(State {
        gold: 0,
        left_gold: 5,
    }));
}
