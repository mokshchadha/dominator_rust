use dominator::{class, clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use once_cell::sync::Lazy;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

static ROOT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "inline-block").style("background-color","#1c54b2").style("padding", "40px")
    }
});

static TEXT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("color", "white").style("font-weight", "bold")
    }
});

static BUTTON_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("display", "block").style("width", "100px").style("margin", "5px")
    }
});

struct App {
    counter: Mutable<i32>,
}

impl App {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            counter: Mutable::new(0),
        })
    }

    fn render(state: &Arc<Self>) -> Dom {
        html!("div", {
            .class(&*ROOT_CLASS).children( [
                html!("div", {
                    .class(&*TEXT_CLASS)
                    .text_signal(state.counter.signal().map(|x| format!("Counter : {}", x)))
                }),

                html!("button", {
                    .class(&*BUTTON_CLASS).text("Decrease")
                    .event(clone!(state => move |_ : events::Click |{
                        state.counter.replace_with(|x| *x-1);
                    }))
                }),

                html!("button", {
                    .class(&*BUTTON_CLASS).text("Increase")
                    .event(clone!(state => move | _:events::Click| {
                        state.counter.replace_with(|x| *x+1);
                    }))
                })
            ])
        })
    }
}

#[wasm_bindgen(start)]
fn main_js() {
    let app = App::new();
    dominator::append_dom(&dominator::body(), App::render(&app));
}
