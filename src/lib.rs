use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::*;

mod common;

#[wasm_bindgen(start)]
pub fn browser_init () {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let content = document.get_element_by_id("content").unwrap();
    let content : &HtmlElement = content.dyn_ref().unwrap();

    content.set_inner_text(format!("hello_world_1: {}, hello_world_2: {}, hello_world_3: {}, hello_world_4: {}",
        common::hello_world_1(),
        common::hello_world_2(),
        common::hello_world_3(),
        common::hello_world_4()
    ).as_str());
}
