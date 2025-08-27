use interactive_fiction_engine::*;

fn main() {
    let mut game = BiasedCoinGame {
        probability: 0.75,
        previous_flips: vec![false],
    };
    biased_coin_game(&mut game);
}

struct BiasedCoinGame {
    probability: f32,
    previous_flips: Vec<bool>,
}

#[derive(Debug)]
enum PlayerChoice {
    GuessFair,
    GuessBiased,
    Wait,
}

fn biased_coin_game(ctx: &mut BiasedCoinGame) {
    showln("Try to guess whether the coin is biased or not.");
    showln("A fair coin has 50/50 chance of landing heads/tails.");
    showln("A biased coin is biased in some direction, by at least 20%.");
    showln("Thus, at least 40/60, or 60/40, but 100/0 at worst.");

    show("Flip history so far: ");
    for flip in &ctx.previous_flips {
        match flip {
            true => show("H"),
            false => show("T"),
        }
    }
    showln("");

    // let player_choice: PlayerChoice = choice! {
    //     "Flip it again" => {
    //         println!("Lets flip another coin");
    //         PlayerChoice::Wait
    //     },
    //     "Guess that it is fair" => PlayerChoice::GuessFair,
    //     "Guess that it is biased" => PlayerChoice::GuessBiased,
    // };
    let player_choice: PlayerChoice = choice(vec![
        ChoiceOption::new("Flip it again", || {
            showln("Let's flip it again");
            PlayerChoice::Wait
        }),
        ChoiceOption::new("Guess that it is fair", || PlayerChoice::GuessFair),
        ChoiceOption::new("Guess that it is biased", || PlayerChoice::GuessBiased),
    ]);

    dbg!(player_choice);
}
