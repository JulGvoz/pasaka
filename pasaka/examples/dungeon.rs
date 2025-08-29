use pasaka::{engine::Engine, runner::cli::CliRunner};

use crate::game::{GameState, caverns};

fn main() {
    Engine::run(
        caverns,
        GameState {
            gold: 0,
            monster: true,
        },
        CliRunner,
    );
}

mod combat {
    use pasaka::{choice::*, engine::*};

    pub struct CombatState<S: 'static> {
        player_hp: i32,
        enemy_hp: i32,
        win_passage: Passage<S>,
        win_state: S,
    }

    impl<S> CombatState<S> {
        pub fn new(player_hp: i32, enemy_hp: i32, win_passage: Passage<S>, win_state: S) -> Self {
            Self {
                player_hp,
                enemy_hp,
                win_passage,
                win_state,
            }
        }
    }

    pub fn combat<S>(state: CombatState<S>) -> Choice {
        Engine::text("You are fighting against a monster!");
        Engine::text(format!("You have {} hp.", state.player_hp));
        Engine::text(format!("It has {} hp.", state.enemy_hp));

        let damage = fastrand::i32(0..=5);
        Engine::text(format!("It is attacking for {damage} damage!"));

        Engine::choice(state)
            .option("Attack it", move |mut state, h| {
                state.enemy_hp -= 10;
                if state.enemy_hp <= 0 {
                    h.passage(state.win_passage, state.win_state)
                } else {
                    state.player_hp -= damage;
                    if state.player_hp <= 0 {
                        h.passage(death, ())
                    } else {
                        h.passage(combat, state)
                    }
                }
            })
            .option("Defend against its attack", move |mut state, h| {
                let damage = damage / 2;
                state.player_hp -= damage;
                if state.player_hp <= 0 {
                    h.passage(death, ())
                } else {
                    h.passage(combat, state)
                }
            })
            .build()
    }

    fn death(_: ()) -> Choice {
        Engine::text("You died fighting against the monster...");

        Engine::choice(()).build()
    }
}

mod game {
    use super::combat::*;

    use pasaka::{choice::*, engine::*};

    pub struct GameState {
        pub gold: i32,
        pub monster: bool,
    }

    pub fn caverns(state: GameState) -> Choice {
        Engine::text("You are exploring the caverns.");
        if state.monster {
            Engine::text("You see a path forwards, but it blocked by a monster");

            Engine::choice(state)
                .option("Engage the monster", |state, h| {
                    let combat_state = CombatState::new(20, 100, path, state);

                    h.passage(combat, combat_state)
                })
                .option("Explore more", |state, h| {
                    Engine::text(
                        "You explore the dungeon more, but don't find anything interesting.",
                    );
                    h.passage(caverns, state)
                })
                .build()
        } else {
            todo!("irrelevant for now")
        }
    }

    fn path(mut state: GameState) -> Choice {
        state.gold += 5;

        Engine::text("You have found some treasure!");
        Engine::text(format!("You now have {} gold.", state.gold));

        Engine::choice(state).build()
    }
}
