use dioxus::prelude::*;
use hikmah_core::library::Library;
use ui::component::library_view::LibraryView;
use ui::MAIN_CSS;

fn main() {
    dioxus_logger::initialize_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let library = use_signal(Library::new);

    rsx! {
        link {
            rel: "stylesheet",
            href: MAIN_CSS
        }

        LibraryView{library}
    }
}

