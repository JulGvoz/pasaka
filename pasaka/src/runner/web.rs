use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::{Passage, engine::Engine};

pub struct WebRunner {
    engine: Engine,
}

impl WebRunner {
    pub fn run(passage: Passage) {
        let prop: WebRunnerProps = WebRunnerProps::new(passage);
        yew::Renderer::<WebRunner>::with_props(prop).render();
    }
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
    Undo,
    Redo,
}

impl Component for WebRunner {
    type Message = Msg;

    type Properties = WebRunnerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let passage = ctx.props().start_passage.clone();
        let engine = Engine::new(passage);
        Self { engine }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Choice(i) => {
                self.engine.update(i);
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
                true
            }
            Msg::Undo => {
                self.engine.undo();
                true
            }
            Msg::Redo => {
                self.engine.redo();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text: Html = self
            .engine
            .current()
            .text
            .iter()
            .map(|line| {
                html! {
                    <span>{line}</span>
                }
            })
            .collect();

        let choices: Html = self
            .engine
            .current()
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
        let undo = ctx.link().callback(|_| Msg::Undo);
        let redo = ctx.link().callback(|_| Msg::Redo);

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
            <p>
                <a href="javascript:void(0)" onclick={undo} disabled={!self.engine.state().can_undo()}>{"Undo"}</a>
                {"|"}
                <a href="javascript:void(0)" onclick={redo} disabled={!self.engine.state().can_redo()}>{"Redo"}</a>
            </p>
            </>
        }
    }
}
