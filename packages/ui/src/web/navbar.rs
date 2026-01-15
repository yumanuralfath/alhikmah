use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        nav {
            id: "navbar",
            div {
                class: "nav-container",

                div {
                    class: "nav-logo",
                    span { class: "logo-icon", "📚" }
                    span { class: "logo-text", "BookReader" }
                }

                div {
                    class: "nav-menu",
                    a { class: "nav-link", href: "#", "Home" }
                    a { class: "nav-link", href: "#", "Library" }
                    a { class: "nav-link", href: "#", "Discover" }
                    a { class: "nav-link", href: "#", "Settings" }
                }

                button {
                    class: "menu-toggle",
                    onclick: move |_| menu_open.set(!menu_open()),
                    "☰"
                }
            }

            if menu_open() {
                div {
                    class: "mobile-menu",
                    a { class: "mobile-link", href: "#", "Home" }
                    a { class: "mobile-link", href: "#", "Library" }
                    a { class: "mobile-link", href: "#", "Discover" }
                    a { class: "mobile-link", href: "#", "Settings" }
                }
            }
        }
    }
}
