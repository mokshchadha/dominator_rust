use dominator::{html, styles, traits::AsStr, DomBuilder};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

#[wasm_bindgen(start)]
pub fn main_js() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Info");
    log::warn!("waring");
    log::error!("Error");

    // let window = web_sys::window().unwrap();
    // let document = window.document().unwrap();
    // let body = document.body().unwrap();

    // let elem: web_sys::Element = document.create_element("div").unwrap();
    // let dom_builder:dominator::DomBuilder<web_sys::Element> = dominator::DomBuilder::new(elem.unchecked_into());
    // let dom = dom_builder.text("This is america").into_dom();

    // dominator::append_dom(&body,dom);

    // let elem = document.create_element("div").unwrap();
    // elem.set_text_content(Some("this is a text"));
    // let dom = dominator::Dom::new(elem.unchecked_into());
    let children_text = vec!["first_txt", "second_txt"];

    dominator::append_dom(
        &dominator::body(),
        html!("div", {
               .children(children_text.into_iter().map(|text|{
                html!("li", {
                    .text(text).apply(add_styles)
                })
               }))
        }),
    );
}

fn add_styles(b: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    b.style("color", "red").style("font-size", "xx-large")
}
