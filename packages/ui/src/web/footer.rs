use dioxus::prelude::*;

const FOOTER_CSS: Asset = asset!("/assets/styling/footer.css");

#[component]
pub fn Footer() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FOOTER_CSS }
        footer {
            id: "footer",
            p { "© 2025 BookReader. Built with Dioxus & Rust 🦀" }
        }
    }
}
