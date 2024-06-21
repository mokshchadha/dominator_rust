use dominator::{Dom, html};

pub struct About {}

impl About {
    pub fn render() -> Dom {
        html!("div" ,{
            .text("About Page")
        })
    }
}
