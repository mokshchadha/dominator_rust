use std::sync::Arc;

use dominator::{Dom, events, html};

pub struct Home {}

impl Home {
    pub fn render() -> Dom {
        let button = Button::new("Click Me", || {});

        html!("div", {
            .text("Home Page")
            .child(button.render())
        })
    }
}

struct Button {
    title: String,
    on_click: Arc<dyn Fn()>,
}

impl Button {
    // Constructor for the Button struct
    fn new(title: impl Into<String>, on_click: impl Fn() + 'static) -> Self {
        Self {
            title: title.into(),
            on_click: Arc::new(on_click),
        }
    }

    // Method to create the button DOM
    fn render(&self) -> Dom {
        let on_click = self.on_click.clone();
        html!("button", {
            .text(&self.title)
            .event(move |_: events::Click| {
                (on_click)();
            })
        })
    }
}
