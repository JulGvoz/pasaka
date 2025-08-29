// Engine
pub mod engine {
    pub trait StoryModule {
        type State;

        fn enter(passage: fn(&mut Engine, &mut Self::State) -> Passage, state: Self::State);
    }

    pub struct Passage {
        _private: (),
    }

    pub struct NextPassage {
        _private: (),
    }

    pub struct Engine;

    impl Engine {
        pub fn start<S>(passage: fn(&mut Self, &mut S) -> Passage, state: S) {
            todo!()
        }

        pub fn choice(&self, f: impl FnOnce(&str) -> NextPassage) -> Passage {
            todo!()
        }

        pub fn text(&mut self, str: impl ToString) {
            todo!()
        }

        // pub fn cross(&mut self, )
    }
}

// Example Game
mod combat {
    use super::engine::*;

    pub struct Module;

    impl StoryModule for Module {
        type State = State;

        fn enter(passage: fn(&mut Engine, &mut Self::State) -> Passage, state: Self::State) {
            todo!()
        }
    }

    pub struct State {
        player_hp: i32,
        enemy_hp: i32,
    }

    impl State {
        pub fn new(player_hp: i32, enemy_hp: i32) -> Self {
            Self {
                player_hp,
                enemy_hp,
            }
        }
    }

    pub fn combat(engine: &mut Engine, state: &mut State) -> Passage {
        engine.text("You are fighting against a monster!");
        engine.text(format!("You have {} hp.", state.player_hp));
        engine.text(format!("It has {} hp.", state.enemy_hp));

        let monster_attack = fastrand::i32(0..5);
        engine.text(format!("It is attacking for {} damage!", monster_attack));

        engine.choice(|s| match s {
            "Attack it" => todo!(),
            "Defend against its attack" => todo!(),
            _ => unreachable!(),
        })
    }
}

mod game {
    use crate::experiment4::combat;

    use super::engine::*;

    struct Module;

    struct State {
        gold: i32,
    }

    fn caverns(engine: &mut Engine, state: &mut State) -> Passage {
        engine.text("You are exploring the caverns. You see a path forwards, but it is blocked by a monster.");

        engine.choice(|s| match s {
            "Engage the monster" => {
                combat::Module::enter(combat::combat, combat::State::new(20, 100));
                todo!()
            }
            "Explore more" => todo!(),
            _ => unreachable!(),
        })
    }

    fn path(engine: &mut Engine, state: &mut State) -> Passage {
        engine.text("You've found some treasure!");
        engine.text(format!("You now have {}.", state.gold));

        todo!()
    }

    #[allow(unused)]
    fn main() {
        Engine::start(caverns, State { gold: 0 });
    }
}
