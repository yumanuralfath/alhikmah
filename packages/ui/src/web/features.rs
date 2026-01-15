use dioxus::prelude::*;

const FEATURES_CSS: Asset = asset!("/assets/styling/features.css");

#[component]
pub fn Features() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FEATURES_CSS }
        section {
            id: "features",
            h2 { class: "section-title", "Why Choose BookReader?" }
            div {
                class: "features-grid",

                FeatureCard {
                    icon: "🔄",
                    title: "Cross-Platform Sync",
                    description: "Seamlessly sync your reading progress across all devices"
                }
                FeatureCard {
                    icon: "🎨",
                    title: "Customizable Reading",
                    description: "Adjust fonts, themes, and layout to your preference"
                }
                FeatureCard {
                    icon: "📝",
                    title: "Notes & Highlights",
                    description: "Take notes and highlight important passages"
                }
                FeatureCard {
                    icon: "📚",
                    title: "Large Format Support",
                    description: "Support for EPUB, PDF, MOBI, and more"
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div {
            class: "feature-card",
            div { class: "feature-icon", "{icon}" }
            h3 { class: "feature-title", "{title}" }
            p { class: "feature-description", "{description}" }
        }
    }
}
