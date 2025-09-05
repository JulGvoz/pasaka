use yew::prelude::*;

use crate::{Passage, engine::Engine};

pub struct WebRunner {
    engine: Engine,
}

#[derive(Properties, PartialEq)]
pub struct WebRunnerProps {
    passage: Passage,
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
        let passage = ctx.props().passage.clone();
        let engine = Engine::new(passage);
        Self { engine }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>
            <div>

            </div>
            </>
        )
    }
}
