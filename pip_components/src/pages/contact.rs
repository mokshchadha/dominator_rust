use dominator::{Dom, html};

pub struct Contact {}

impl Contact {
    pub fn render() -> Dom {
        html!("div", {.text("Contact Page")})
    }
}
