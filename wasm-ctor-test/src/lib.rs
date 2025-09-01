use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_entry() {}

pub struct Entry {
    f: fn() -> String,
}

impl Entry {
    pub const fn new(f: fn() -> String) -> Entry {
        Entry { f }
    }
}

fn marked_fn() -> String {
    String::from("hello, wasm")
}

inventory::collect!(Entry);

inventory::submit! {
    Entry::new(marked_fn)
}
