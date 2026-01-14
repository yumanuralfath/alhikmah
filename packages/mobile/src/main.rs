use dioxus::prelude::*;
use ui::{Features, Footer, Hero, Navbar, GLOBAL_CSS};

fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
         document::Link { rel: "stylesheet", href: GLOBAL_CSS }
         div {
             class: "app-container",
             Navbar {  }
             Hero {  }
             Features {  }
             Footer {  }
         }
    }
}
