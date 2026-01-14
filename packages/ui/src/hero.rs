use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");

#[component]
pub fn Hero() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }
        section {
            id: "hero",
            div {
                class: "hero-content",
                h1 {
                    class: "hero-title",
                    "Your Digital Library"
                    br {}
                    span { class: "hero-highlight", "Anywhere, Anytime" }
                }
                p {
                    class: "hero-description",
                    "Read your favorite books across all your devices with our powerful multi-platform ebook reader. Sync your progress, highlights, and notes seamlessly."
                }
                div {
                    class: "hero-buttons",
                    button {
                        class: "btn btn-primary",
                        onclick: |_| {
                            // Handle start reading
                        },
                        "Start Reading"
                    }
                    button {
                        class: "btn btn-secondary",
                        onclick: |_| {
                            // Handle browse library
                        },
                        "Browse Library"
                    }
                }
            }

            div {
                class: "hero-image",
                div {
                    class: "book-stack",
                    div { class: "book book-1", "📖" }
                    div { class: "book book-2", "📕" }
                    div { class: "book book-3", "📘" }
                }
            }
        }
    }
}

