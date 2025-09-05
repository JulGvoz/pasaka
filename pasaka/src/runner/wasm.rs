use futures::StreamExt;
use gloo::events::EventListener;
use web_sys::{Document, Element, Window};

use crate::{choice::PassageResult, engine::Engine};

pub struct WasmRunner {
    engine: Engine,

    window: Window,
    document: Document,
    text: Element,
    choice: Element,
    save: Option<Element>,
    load: Option<Element>,

    save_listener: Option<EventListener>,
    load_listener: Option<EventListener>,
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
        let document = window.document().expect("window should have a document");

        let text = document
            .get_element_by_id(&text_id.to_string())
            .expect("no text container found");

        let choice = document
            .get_element_by_id(&choices_id.to_string())
            .expect("no choice container found");

        let save: Option<Element> = document.get_element_by_id(&save_id.to_string());
        let load: Option<Element> = document.get_element_by_id(&load_id.to_string());

        let save_listener = None;
        let load_listener = None;

        Self {
            engine,

            window,
            document,
            text,
            choice,
            save,
            load,

            save_listener,
            load_listener,
        }
    }

    pub async fn run(&mut self) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        if let Some(save) = &self.save {
            let listener = EventListener::new(&save, "click", |_event: &web_sys::Event| {
                // todo: data escapes
                // self.save();
            });
            self.save_listener = Some(listener)
        }
        if let Some(load) = &self.load {
            let listener = EventListener::new(&load, "click", |_event| {
                // todo
                // self.load();
            });
            self.load_listener = Some(listener)
        }

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
        self.text.set_inner_html(&text);
    }

    pub fn show_choice(&self, passage: &PassageResult) -> Vec<Element> {
        self.choice.set_inner_html("");

        let mut label_links = Vec::new();
        for label in &passage.labels {
            let list_item = self.document.create_element("li").unwrap();
            let label_elem = self.document.create_element("a").unwrap();
            label_elem.set_text_content(Some(&label));
            label_elem.set_attribute("href", "#").unwrap();

            list_item.append_child(&label_elem).unwrap();
            self.choice.append_child(&list_item).unwrap();
            label_links.push(label_elem);
        }

        label_links
    }

    async fn make_choice(&self, choices: &[Element]) -> usize {
        let (tx, mut rx) = ::futures::channel::mpsc::unbounded();

        let mut listeners = Vec::new();
        for (i, choice) in choices.iter().enumerate() {
            let tx = tx.clone();
            let listener = EventListener::once(&choice, "click", move |_event| {
                let _ = tx.unbounded_send(i);
            });
            listeners.push(listener);
        }

        let index = async move {
            let index = rx.next().await.expect("choice should be made");
            drop(rx);
            listeners.clear();
            index
        };
        index.await
    }

    pub fn save(&self) {
        let state = self.engine.state();
        let Ok(Some(storage)) = self.window.local_storage() else {
            return;
        };
        let Ok(json) = serde_json::to_string(state) else {
            return;
        };
        let _ = storage.set_item("save", &json);
    }

    pub fn load(&mut self) {
        let Ok(Some(storage)) = self.window.local_storage() else {
            return;
        };
        let Ok(Some(json)) = storage.get_item("save") else {
            return;
        };
        let Ok(state) = serde_json::from_str(&json) else {
            return;
        };
        self.engine.load_state(state);
    }
}
