use crate::{
    choice::PassageResult,
    engine::{Engine, EngineState},
    runner::{RenderResult, Runner},
};

pub struct WasmRunner;

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

        let elem = document.create_element("p").unwrap();
        elem.set_inner_html(&text);

        body.append_child(&elem).unwrap();

        todo!()
    }

    async fn save(&mut self, value: &EngineState) -> bool {
        todo!()
    }

    async fn load(&mut self) -> Option<EngineState> {
        todo!()
    }
}
