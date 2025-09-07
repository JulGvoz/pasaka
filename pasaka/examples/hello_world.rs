use pasaka::*;

#[passage]
fn HelloWorld(mut h: PassageHandle, state: ()) -> PassageResult {
    h.text("Hello, world!");

    h.choice().build(state)
}

pub fn main() {
    CliRunner::run(HelloWorld.with_state(()));
}
