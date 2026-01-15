use dioxus::prelude::*;
use hikmah_core::android::library_android;
use ui::android::file_browser_android::FileBrowser;
use ui::android::library_view_android::LibraryView;
use ui::MAIN_CSS;

fn main() {
    dioxus_logger::initialize_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let library = use_signal(library_android::LibraryAndroid::new);

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
