pub mod engine {
    pub struct Engine {
        _private: (),
    }

    impl Engine {
        pub fn text(s: impl ToString) {
            todo!()
        }
    }

    pub struct Choice {
        _private: (),
    }

    pub struct ChoiceHandle {
        _private: (),
    }

    impl ChoiceHandle {
        pub fn passage<S>(self, p: Passage<S>, s: &mut S) -> ChoiceResult {
            todo!()
        }
    }

    pub struct ChoiceResult {
        _private: (),
    }

    impl Choice {
        pub fn new(f: impl FnOnce(ChoiceHandle, &str) -> ChoiceResult) -> Choice {
            let handle = ChoiceHandle { _private: () };
            let result = f(handle, "example input");
            Choice { _private: () }
        }
    }

    pub type Passage<S> = fn(&mut S) -> Choice;
}

mod combat {
    use super::engine::*;

    pub struct CombatState<'a, S> {
        player_hp: i32,
        enemy_hp: i32,
        win_passage: Passage<S>,
        win_state: &'a mut S,
    }

    impl<'a, S> CombatState<'a, S> {
        pub fn new(
            player_hp: i32,
            enemy_hp: i32,
            win_passage: Passage<S>,
            win_state: &'a mut S,
        ) -> Self {
            Self {
                player_hp,
                enemy_hp,
                win_passage,
                win_state,
            }
        }
    }

    pub fn combat<S>(state: &mut CombatState<S>) -> Choice {
        Engine::text("You are fighting against a monster!");
        Engine::text(format!("You have {} hp.", state.player_hp));
        Engine::text(format!("It has {} hp.", state.enemy_hp));

        let damage = fastrand::i32(0..=5);
        Engine::text(format!("It is attacking for {damage} damage!"));

        Choice::new(|h, s| match s {
            "Attack it" => {
                state.enemy_hp -= 10;
                if state.enemy_hp <= 0 {
                    h.passage(state.win_passage, state.win_state)
                } else {
                    state.player_hp -= damage;
                    if state.player_hp <= 0 {
                        h.passage(death, &mut ())
                    } else {
                        h.passage(combat, state)
                    }
                }
            }
            "Defend against its attack" => todo!(),
            _ => unreachable!(),
        })
    }

    fn death(state: &mut ()) -> Choice {
        todo!()
    }
}

mod game {
    use crate::experiment5::combat::*;

    use super::engine::*;

    struct GameState {
        gold: i32,
        monster: bool,
    }

    fn caverns(state: &mut GameState) -> Choice {
        Engine::text("You are exploring the caverns.");
        if state.monster {
            Engine::text("You see a path forwards, but it blocked by a monster");

            Choice::new(|h, s| match s {
                "Engage the monster" => {
                    let mut combat_state = CombatState::new(20, 100, path, state);

                    h.passage(combat, &mut combat_state)
                }
                "Explore more" => h.passage(caverns, state),
                _ => unreachable!(),
            })
        } else {
            todo!()
        }
    }

    fn path(state: &mut GameState) -> Choice {
        state.gold += 5;

        Engine::text("You have found some treasure!");
        Engine::text(format!("You now have {} gold.", state.gold));

        todo!()
    }
}
