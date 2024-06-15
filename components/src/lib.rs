use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;

#[derive(Clone)]
struct Tab {
    label: &'static str,
    content: Rc<dyn Fn() -> Dom>,
}

struct TabView {
    tabs: Vec<Tab>,
    selected_tab: Mutable<usize>,
}

impl TabView {
    fn new(tabs: Vec<Tab>) -> Rc<Self> {
        Rc::new(Self {
            tabs,
            selected_tab: Mutable::new(0),
        })
    }

    fn render(self: Rc<Self>) -> Dom {
        let tabs = self.tabs.clone();
        let selected_tab = self.selected_tab.clone();

        html!("div", {
            .children(&mut [
                html!("div", {
                    .class("tab-header")
                    .children(tabs.iter().enumerate().map(|(index, tab)| {
                        let selected_tab = selected_tab.clone();
                        html!("button", {
                            .text(tab.label)
                            .class_signal("active", selected_tab.signal().map(move |selected_index| selected_index == index))
                            .event(clone!(selected_tab => move |_: events::Click| {
                                selected_tab.set(index);
                            }))
                        })
                    }).collect::<Vec<_>>())
                }),
                html!("div", {
                    .class("tab-content")
                    .child_signal(selected_tab.signal().map(move |selected_index| {
                        Some((tabs[selected_index].content)())
                    }))
                })
            ])
        })
    }
}

fn main() {
    let tab_view = TabView::new(vec![
        Tab {
            label: "Tab 1",
            content: Rc::new(|| html!("div", {
                .text("Content of Tab 1")
            })),
        },
        Tab {
            label: "Tab 2",
            content: Rc::new(|| html!("div", {
                .text("Content of Tab 2")
            })),
        },
        Tab {
            label: "Tab 3",
            content: Rc::new(|| html!("div", {
                .text("Content of Tab 3")
            })),
        },
    ]);

    dominator::append_dom(&dominator::get_id("app"), tab_view.render());
}
