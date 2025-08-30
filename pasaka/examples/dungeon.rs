use pasaka::{engine::Engine, runner::cli::CliRunner};

use crate::game::{GameState, caverns};

fn main() {
    smol::block_on(Engine::run(
        caverns,
        GameState {
            gold: 0,
            monster: true,
        },
        CliRunner,
    ))
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

    pub fn combat<S>(engine: &mut Engine, state: CombatState<S>) -> Choice {
        engine.text("You are fighting against a monster!");
        engine.text(format!("You have {} hp.", state.player_hp));
        engine.text(format!("It has {} hp.", state.enemy_hp));

        let damage = fastrand::i32(0..=5);
        engine.text(format!("It is attacking for {damage} damage!"));

        engine
            .choice()
            .option("Attack it", move |mut state: CombatState<S>, mut h| {
                state.enemy_hp -= 10;
                h.text("You deal 10 damage to the monster.");
                if state.enemy_hp <= 0 {
                    h.text("You have defeated the monster!");
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
            .option("Defend against its attack", move |mut state, mut h| {
                let original_damage = damage;
                let damage = 0.max(damage - 3);
                state.player_hp -= damage;
                h.text(format!("You block for {} damage", original_damage - damage));
                if state.player_hp <= 0 {
                    h.passage(death, ())
                } else {
                    h.passage(combat, state)
                }
            })
            .build(state)
    }

    fn death(engine: &mut Engine, _: ()) -> Choice {
        engine.text("You died fighting against the monster...");

        engine.choice().build(())
    }
}

mod game {
    use super::combat::*;

    use pasaka::{choice::*, engine::*};

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct GameState {
        pub gold: i32,
        pub monster: bool,
    }

    pub fn caverns(engine: &mut Engine, state: GameState) -> Choice {
        engine.text("You are exploring the caverns.");
        if state.monster {
            engine.text("You see a path forwards, but it blocked by a monster");

            engine
                .choice()
                .option("Engage the monster", |state, h| {
                    let combat_state = CombatState::new(20, 100, path, state);

                    h.passage(combat, combat_state)
                })
                .option("Explore more", |state, mut h| {
                    h.text("You explore the dungeon more, but don't find anything interesting.");
                    h.passage(caverns, state)
                })
                .build(state)
        } else {
            todo!("irrelevant for now")
        }
    }

    fn path(engine: &mut Engine, mut state: GameState) -> Choice {
        state.gold += 5;

        engine.text("You have found some treasure!");
        engine.text(format!("You now have {} gold.", state.gold));

        engine.choice().build(state)
    }
}
