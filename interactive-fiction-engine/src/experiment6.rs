pub mod engine {
    use std::cell::RefCell;

    use dialoguer::Select;

    thread_local! {
        static TEXT_BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
    }

    pub struct Engine {
        _private: (),
    }

    impl Engine {
        pub fn text(s: impl ToString) {
            // println!("{}", s.to_string())
            TEXT_BUF.with_borrow_mut(|buf| {
                buf.push(s.to_string());
            })
        }

        fn take_text() -> Vec<String> {
            TEXT_BUF.with_borrow_mut(|buf| std::mem::take(buf))
        }

        pub fn choice<S>(state: S) -> ChoiceBuilder<S> {
            ChoiceBuilder {
                state,
                options: Vec::new(),
            }
        }

        pub fn run<S>(passage: Passage<S>, state: S) {
            let mut current: Box<dyn FnOnce() -> Choice> = Box::new(move || passage(state));

            loop {
                let choice = current();

                for line in &choice.text {
                    println!("{line}");
                }
                println!();

                if choice.labels.is_empty() {
                    break;
                }

                let index = Select::new()
                    .default(0)
                    .items(choice.labels)
                    .interact()
                    .unwrap();

                let handle = ChoiceHandle { _private: () };
                let result = (choice.action)(index, handle);
                current = result.next_passage;
            }
        }
    }

    pub struct Choice {
        text: Vec<String>,
        labels: Vec<String>,
        action: Box<dyn FnOnce(usize, ChoiceHandle) -> ChoiceResult>,
    }

    struct ChoiceOption<S> {
        label: String,
        on_choose: Box<dyn FnOnce(S, ChoiceHandle) -> ChoiceResult>,
    }

    pub struct ChoiceBuilder<S: 'static> {
        state: S,
        options: Vec<ChoiceOption<S>>,
    }

    impl<S> ChoiceBuilder<S> {
        pub fn option(
            mut self,
            label: impl ToString,
            f: impl FnOnce(S, ChoiceHandle) -> ChoiceResult + 'static,
        ) -> Self {
            let option = ChoiceOption {
                label: label.to_string(),
                on_choose: Box::new(f),
            };
            self.options.push(option);

            self
        }

        pub fn build(mut self) -> Choice {
            let text = Engine::take_text();
            let labels = self
                .options
                .iter_mut()
                .map(|o| std::mem::take(&mut o.label))
                .collect();

            let action = Box::new(move |index, handle| {
                let option = self
                    .options
                    .into_iter()
                    .nth(index)
                    .expect("selected option should be within bounds of possible options");

                (option.on_choose)(self.state, handle)
            });

            Choice {
                text,
                labels,
                action,
            }
        }
    }

    pub struct ChoiceHandle {
        _private: (),
    }

    impl ChoiceHandle {
        pub fn passage<S: 'static>(self, p: Passage<S>, s: S) -> ChoiceResult {
            ChoiceResult {
                next_passage: Box::new(move || p(s)),
            }
        }
    }

    pub struct ChoiceResult {
        next_passage: Box<dyn FnOnce() -> Choice>,
    }

    pub type Passage<S> = fn(S) -> Choice;
}

mod combat {
    use super::engine::*;

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

    use super::engine::*;

    struct GameState {
        gold: i32,
        monster: bool,
    }

    fn caverns(state: GameState) -> Choice {
        Engine::text("You are exploring the caverns.");
        if state.monster {
            Engine::text("You see a path forwards, but it blocked by a monster");

            Engine::choice(state)
                .option("Engage the monster", |state, h| {
                    let combat_state = CombatState::new(20, 100, path, state);

                    h.passage(combat, combat_state)
                })
                .option("Explore more", |state, h| h.passage(caverns, state))
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

    #[allow(unused)]
    fn main() {
        Engine::run(
            caverns,
            GameState {
                gold: 0,
                monster: true,
            },
        );
    }
}
