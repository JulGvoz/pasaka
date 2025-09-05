use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::{Passage, choice::PassageResult, engine::Engine};

pub struct WebRunner {
    engine: Engine,
    current: PassageResult,
}

#[derive(Properties, PartialEq)]
pub struct WebRunnerProps {
    start_passage: Passage,
}

impl WebRunnerProps {
    pub fn new(start_passage: Passage) -> Self {
        Self { start_passage }
    }
}

pub enum Msg {
    Choice(usize),
    Save,
    Load,
}

impl Component for WebRunner {
    type Message = Msg;

    type Properties = WebRunnerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let passage = ctx.props().start_passage.clone();
        let engine = Engine::new(passage);
        let current = engine.step();
        Self { engine, current }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Choice(i) => {
                if i >= self.current.labels.len() {
                    return false;
                }
                // SAFETY: self.current is written back to before leaving this scope
                // furthermore, self is not used anywhere in the next 3 lines
                let current = unsafe { std::ptr::read(&self.current) };
                let choice = (current.action)(i);
                self.engine.update(choice);
                // SAFETY: &mut is always safe to write to.
                unsafe { std::ptr::write(&mut self.current, self.engine.step()) };
                true
            }
            Msg::Save => {
                let state = self.engine.state().clone();
                let _ = LocalStorage::set("save", state);
                false
            }
            Msg::Load => {
                let Ok(state) = LocalStorage::get("save") else {
                    return false;
                };
                self.engine.load_state(state);
                self.current = self.engine.step();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text: Html = self
            .current
            .text
            .iter()
            .map(|line| {
                html! {
                    <span>{line}</span>
                }
            })
            .collect();

        let choices: Html = self
            .current
            .labels
            .iter()
            .enumerate()
            .map(|(i, label)| {
                let onclick = ctx.link().callback(move |_| Msg::Choice(i));
                html! {
                    <li key={i}><a href="javascript:void(0)" {onclick}>{format!("{label}")}</a></li>
                }
            })
            .collect();

        let save = ctx.link().callback(|_| Msg::Save);
        let load = ctx.link().callback(|_| Msg::Load);

        html! {
            <>
            <p>
                {text}
            </p>
            <div>
                <ul>
                {choices}
                </ul>
            </div>
            <hr />
            <p><a href="javascript:void(0)" onclick={save}>{"Save"}</a></p>
            <p><a href="javascript:void(0)" onclick={load}>{"Load"}</a></p>
            </>
        }
    }
}
