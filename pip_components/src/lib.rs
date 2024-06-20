use std::collections::HashMap;
use std::sync::Arc;

use dominator::{clone, Dom, events, html};
use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main_js() {
    dominator::append_dom(&dominator::body(), App::new().render());
}

struct App {
    route: Arc<Mutable<String>>,
    router: Router,
    navbar: Navbar,
}

impl App {
    fn new() -> Self {
        let route = Arc::new(Mutable::new(get_route()));
        let route_clone = Arc::clone(&route);

        let window = window().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            let new_route = get_route();
            route_clone.set(new_route);
        }) as Box<dyn FnMut()>);

        window.set_onhashchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget(); // Prevent the closure from being dropped

        Self {
            route: Arc::clone(&route),
            router: Router::new(Arc::clone(&route)),
            navbar: Navbar::new(Arc::clone(&route)),
        }
    }

    fn render(self) -> Dom {
        let route = Arc::clone(&self.route);
        html!("div", {
            .child(self.navbar.render())
            .child_signal(self.route.signal_cloned().map(move |route| {
                Some(self.router.render(&route))
            }))
        })
        // html!("div", {
        //     .child(self.navbar.render())
        //
        // })
    }
}

struct Router {
    components: HashMap<String, Box<dyn Fn() -> Dom>>,
    route: Arc<Mutable<String>>,
}

impl Router {
    fn new(route: Arc<Mutable<String>>) -> Self {
        let comp = create_components_map();
        Self {
            components: comp,
            route,
        }
    }

    fn render(&self, route: &String) -> Dom {
        match self.components.get(route) {
            Some(comp) => comp(),
            None => not_found(),
        }
    }
}
struct Navbar {
    route: Arc<Mutable<String>>,
}

impl Navbar {
    fn new(route: Arc<Mutable<String>>) -> Self {
        Self { route }
    }

    fn render(self) -> Dom {
        html!("nav", {
            .children(vec![
                self.link("Home", "/"),
                self.link("About", "/about"),
                self.link("Contact", "/contact")
            ])
        })
    }

    fn link(&self, title: &str, path: &str) -> Dom {
        let route = self.route.clone();
        let path = path.to_string();
        html!("a", {
            .attr("href", &format!("#{}", path))
            .text(title)
            .event(clone!(route => move |_: events::Click| {
                route.set(path.clone());
            }))
        })
    }
}

fn create_components_map() -> HashMap<String, Box<dyn Fn() -> Dom>> {
    let mut components = HashMap::new();
    components.insert(
        "/".to_string(),
        Box::new(Home::render) as Box<dyn Fn() -> Dom>,
    );
    components.insert(
        "/about".to_string(),
        Box::new(About::render) as Box<dyn Fn() -> Dom>,
    );
    components.insert(
        "/contact".to_string(),
        Box::new(Contact::render) as Box<dyn Fn() -> Dom>,
    );
    components.insert("".to_string(), Box::new(not_found) as Box<dyn Fn() -> Dom>);
    components
}

fn not_found() -> Dom {
    html!("div", {
        .text("Page Not Found")
    })
}

fn get_route() -> String {
    window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_else(|| "/#".to_string())
        .trim_start_matches("#")
        .to_string()
}

struct Home {}

impl Home {
    fn render() -> Dom {
        html!("div", {
            .text("Home Page")
        })
    }
}

struct About {}

impl About {
    fn render() -> Dom {
        html!("div" ,{
            .text("About Page")
        })
    }
}

struct Contact {}

impl Contact {
    fn render() -> Dom {
        html!("div", {.text("Contact Page")})
    }
}
