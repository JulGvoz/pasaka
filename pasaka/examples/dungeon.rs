use pasaka::{engine::Engine, runner::cli::CliRunner};

use crate::game::{Caverns, GameState};

fn main() {
    smol::block_on(Engine::run(
        Caverns,
        GameState {
            gold: 0,
            monster: true,
        },
        CliRunner,
    ))
}

mod combat {
    use pasaka::{Passage, choice::*};
    use pasaka_macro::passage;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CombatState {
        player_hp: i32,
        enemy_hp: i32,
        on_win: Passage,
    }

    impl CombatState {
        pub fn new(player_hp: i32, enemy_hp: i32, on_win: Passage) -> Self {
            Self {
                player_hp,
                enemy_hp,
                on_win,
            }
        }
    }

    #[passage]
    pub fn Combat(mut h: PassageHandle, state: CombatState) -> PassageResult {
        h.text("You are fighting against a monster!")
            .text(format!("You have {} hp.", state.player_hp))
            .text(format!("It has {} hp.", state.enemy_hp));

        let damage = fastrand::i32(0..=5);
        h.text(format!("It is attacking for {damage} damage!"));

        h.choice()
            .option("Attack it", move |mut state: CombatState, mut h| {
                state.enemy_hp -= 10;
                h.text("You deal 10 damage to the monster.");
                if state.enemy_hp <= 0 {
                    h.text("You have defeated the monster!");
                    // h.passage(state.win_passage, state.win_state)
                    h.passage_with_state(state.on_win)
                } else {
                    state.player_hp -= damage;
                    if state.player_hp <= 0 {
                        h.passage(Death, ())
                    } else {
                        h.passage(Combat, state)
                    }
                }
            })
            .option("Defend against its attack", move |mut state, mut h| {
                let original_damage = damage;
                let damage = 0.max(damage - 3);
                state.player_hp -= damage;
                h.text(format!("You block for {} damage", original_damage - damage));
                if state.player_hp <= 0 {
                    h.passage(Death, ())
                } else {
                    h.passage(Combat, state)
                }
            })
            .build(state)
    }

    #[passage]
    fn Death(mut h: PassageHandle, _: ()) -> PassageResult {
        h.text("You died fighting against the monster...");

        h.choice().build(())
    }
}

mod game {
    use super::combat::*;

    use pasaka::choice::*;
    use pasaka_macro::passage;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct GameState {
        pub gold: i32,
        pub monster: bool,
    }

    #[passage]
    pub fn Caverns(mut h: PassageHandle, state: GameState) -> PassageResult {
        h.text("You are exploring the caverns.");
        if state.monster {
            h.text("You see a path forwards, but it blocked by a monster");

            h.choice()
                .option("Engage the monster", |state, h| {
                    let combat_state = CombatState::new(20, 100, Path.with_state(state));

                    h.passage(Combat, combat_state)
                })
                .option("Explore more", |state, mut h| {
                    h.text("You explore the dungeon more, but don't find anything interesting.");
                    h.passage(Caverns, state)
                })
                .build(state)
        } else {
            todo!("irrelevant for now")
        }
    }

    #[passage]
    fn Path(mut h: PassageHandle, mut state: GameState) -> PassageResult {
        state.gold += 5;

        h.text("You have found some treasure!");
        h.text(format!("You now have {} gold.", state.gold));

        h.choice().build(state)
    }
}
