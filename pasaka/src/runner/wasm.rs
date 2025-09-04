use futures::StreamExt;
use gloo::events::EventListener;
use web_sys::Element;

use crate::{
    choice::PassageResult,
    engine::{Engine, EngineState},
    runner::{RenderResult, Runner},
};

pub struct WasmRunner {
    text_container_id: String,
    choices_container_id: String,
}

impl WasmRunner {
    pub fn new(text_id: impl ToString, choices_id: impl ToString) -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        Self {
            text_container_id: text_id.to_string(),
            choices_container_id: choices_id.to_string(),
        }
    }

    fn make_choice(&mut self, choices: &[Element]) -> impl std::future::Future<Output = usize> {
        let (tx, mut rx) = ::futures::channel::mpsc::channel(1);

        let mut listeners = Vec::new();
        for (i, choice) in choices.iter().enumerate() {
            let mut tx = tx.clone();
            let listener = EventListener::once(&choice, "click", move |_event| {
                let _ = tx.try_send(i);
            });
            listeners.push(listener);
        }

        async move {
            let index = rx.next().await.expect("choice should be made");
            drop(rx);
            drop(listeners);
            index
        }
    }
}

#[allow(refining_impl_trait)]
impl Runner for WasmRunner {
    async fn render(
        &mut self,
        _engine: &mut Engine,
        prev_text: &[String],
        choice: PassageResult,
    ) -> RenderResult {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("document should have a body");

        let mut text = String::new();
        for line in prev_text {
            text.push_str(line);
            text.push_str("<br />");
        }
        text.push_str("<br />");
        for line in &choice.text {
            text.push_str(line);
            text.push_str("<br />");
        }
        text.push_str("<br />");

        let text_elem = document
            .get_element_by_id(&self.text_container_id)
            .expect("no text container found");
        text_elem.set_inner_html(&text);

        let choice_elem = document
            .get_element_by_id(&self.choices_container_id)
            .expect("no choice container found");
        choice_elem.set_inner_html("");

        if choice.labels.is_empty() {
            return RenderResult::Exit;
        }

        let mut label_links = Vec::new();
        for label in choice.labels {
            let list_item = document.create_element("li").unwrap();
            let label_elem = document.create_element("a").unwrap();
            label_elem.set_text_content(Some(&label));
            label_elem.set_attribute("href", "#").unwrap();

            list_item.append_child(&label_elem).unwrap();
            choice_elem.append_child(&list_item).unwrap();
            label_links.push(label_elem);
        }

        let index = self.make_choice(&label_links).await;

        RenderResult::Choice((choice.action)(index))
    }

    async fn save(&mut self, value: &EngineState) -> bool {
        todo!()
    }

    async fn load(&mut self) -> Option<EngineState> {
        todo!()
    }
}
