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
}

#[allow(refining_impl_trait)]
impl Runner for WasmRunner {
    async fn render(
        &mut self,
        engine: &mut Engine,
        prev_text: &[String],
        choice: PassageResult,
    ) -> RenderResult {
        web_sys::console::log_1(&"render passage".into());
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("document should have a body");
        let body = document.body().expect("document should have a body");

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

        for label in choice.labels {
            let list_item = document.create_element("li").unwrap();
            let label_elem = document.create_element("a").unwrap();
            label_elem.set_text_content(Some(&label));

            list_item.append_child(&label_elem).unwrap();
            choice_elem.append_child(&list_item).unwrap();
        }

        todo!()
    }

    async fn save(&mut self, value: &EngineState) -> bool {
        todo!()
    }

    async fn load(&mut self) -> Option<EngineState> {
        todo!()
    }
}
