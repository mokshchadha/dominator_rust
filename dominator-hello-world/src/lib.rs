use dominator::html;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js(){
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Info");
    log::warn!("waring");
    log::error!("Error");

    dominator::append_dom(&dominator::body(), html!("div", {.text("Hello world!")}));
}