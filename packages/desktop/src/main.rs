use dioxus::prelude::*;
use hikmah_core::library;
use ui::component::file_browser::FileBrowser;
use ui::component::library_view::LibraryView;
use ui::MAIN_CSS;

fn main() {
    dioxus_logger::initialize_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let library = use_signal(library::Library::new);

    rsx! {
        link {
            rel: "stylesheet",
            href: MAIN_CSS
        }

        if library.read().browsing_mode {
            FileBrowser { library }
        } else {
            LibraryView { library }
        }
    }
}
