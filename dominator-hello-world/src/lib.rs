use dominator::html;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js(){
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Info");
    log::warn!("waring");
    log::error!("Error");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let elem = document.create_element("div").unwrap();
    elem.set_text_content(Some("this is a text"));
    let dom = dominator::Dom::new(elem.unchecked_into());


   // dominator::append_dom(&dominator::body(), html!("div", {.text("Hello world!")}));
   dominator::append_dom(&body,dom);
}