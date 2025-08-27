use interactive_fiction_engine::*;

fn main() {
    let fair = fastrand::bool();
    let bias = if fair {
        0.5
    } else {
        loop {
            let bias = fastrand::f32();
            if bias <= 0.4 || bias >= 0.6 {
                break bias;
            }
        }
    };

    let mut game = BiasedCoinGame {
        probability: bias,
        previous_flips: vec![],
    };
    game.flip_coin();
    biased_coin_game(&mut game);
}

#[derive(Debug)]
struct BiasedCoinGame {
    probability: f32,
    previous_flips: Vec<bool>,
}

impl BiasedCoinGame {
    fn flip_coin(&mut self) -> bool {
        let result = self.probability <= fastrand::f32();
        self.previous_flips.push(result);
        result
    }
}

#[derive(Debug)]
enum PlayerChoice {
    GuessFair,
    GuessBiased,
    Flip,
}

fn biased_coin_game(ctx: &mut BiasedCoinGame) {
    start();
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
        ChoiceOption::new("Flip it again", || PlayerChoice::Flip),
        ChoiceOption::new("Guess that it is fair", || PlayerChoice::GuessFair),
        ChoiceOption::new("Guess that it is biased", || PlayerChoice::GuessBiased),
    ]);

    let correct_guess = match player_choice {
        PlayerChoice::Flip => {
            show("The result is: ");
            let flip = ctx.flip_coin();
            match flip {
                true => show("H"),
                false => show("T"),
            }
            showln("");

            wait();
            return biased_coin_game(ctx);
        }
        PlayerChoice::GuessFair => ctx.probability == 0.5,
        PlayerChoice::GuessBiased => ctx.probability != 0.5,
    };

    showln(format!("The bias was: {:.3}", ctx.probability));

    if correct_guess {
        showln("You guessed correctly! Success!");
    } else {
        showln("You guessed incorrectly :(");
    }
}
