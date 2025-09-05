use futures::StreamExt;
use gloo::events::EventListener;
use web_sys::{Document, Element};

use crate::{choice::PassageResult, engine::Engine};

pub struct WasmRunner {
    engine: Engine,

    text_elem: Element,
    choice_elem: Element,
    save_id: String,
    load_id: String,

    document: Document,

    choice_listeners: Vec<EventListener>,
}

impl WasmRunner {
    pub fn new(
        engine: Engine,
        text_id: impl ToString,
        choices_id: impl ToString,
        save_id: impl ToString,
        load_id: impl ToString,
    ) -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("document should have a body");

        let text_elem = document
            .get_element_by_id(&text_id.to_string())
            .expect("no text container found");

        let choice_elem = document
            .get_element_by_id(&choices_id.to_string())
            .expect("no choice container found");

        Self {
            engine,
            text_elem,
            choice_elem,
            save_id: save_id.to_string(),
            load_id: load_id.to_string(),

            document,

            choice_listeners: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        loop {
            let passage = self.engine.step();

            self.show_text(&passage);

            let choices = self.show_choice(&passage);
            let index = self.make_choice(&choices).await;
            let result = (passage.action)(index);
            self.engine.update(result);
        }
    }

    pub fn show_text(&self, passage: &PassageResult) {
        let mut text = String::new();
        for line in &passage.text {
            text.push_str(line);
            text.push_str("<br />");
        }
        self.text_elem.set_inner_html(&text);
    }

    pub fn show_choice(&self, passage: &PassageResult) -> Vec<Element> {
        self.choice_elem.set_inner_html("");

        let mut label_links = Vec::new();
        for label in &passage.labels {
            let list_item = self.document.create_element("li").unwrap();
            let label_elem = self.document.create_element("a").unwrap();
            label_elem.set_text_content(Some(&label));
            label_elem.set_attribute("href", "#").unwrap();

            list_item.append_child(&label_elem).unwrap();
            self.choice_elem.append_child(&list_item).unwrap();
            label_links.push(label_elem);
        }

        label_links
    }

    async fn make_choice(&mut self, choices: &[Element]) -> usize {
        let (tx, mut rx) = ::futures::channel::mpsc::channel(1);

        for (i, choice) in choices.iter().enumerate() {
            let mut tx = tx.clone();
            let listener = EventListener::once(&choice, "click", move |_event| {
                let _ = tx.try_send(i);
            });
            self.choice_listeners.push(listener);
        }

        let index = async move {
            let index = rx.next().await.expect("choice should be made");
            drop(rx);
            self.choice_listeners.clear();
            index
        };
        index.await
    }
}
