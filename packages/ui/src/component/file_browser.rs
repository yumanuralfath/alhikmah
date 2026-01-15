use dioxus::prelude::*;
use hikmah_core::library::Library;
use hikmah_core::models::FileEntry;

#[component]
pub fn FileBrowser(library: Signal<Library>) -> Element {
    let lib = library.read();

    rsx! {
        div { class: "container",
            BrowserHeader { library }

            div { class: "toolbar",
                button {
                    onclick: move |_| library.write().go_up(),
                    disabled: lib.current_path.parent().is_none(),
                    "⬆️ Up"
                }
            }

            main { class: "file-list",
                for (index, entry) in lib.file_list.iter().enumerate() {
                    FileItem { entry: entry.clone(), index, library }
                }

                if lib.file_list.is_empty() {
                    EmptyFileList {}
                }
            }

            if let Some(err) = &lib.err {
                ErrorBanner { error: err.clone(), library }
            }
        }
    }
}

#[component]
fn BrowserHeader(library: Signal<Library>) -> Element {
    let lib = library.read();

    rsx! {
        header {
            button {
                onclick: move |_| library.write().cancel_browsing(),
                "← Back to Library"
            }
            h1 { "Select Ebook File" }
            p { class: "current-path", "{lib.current_path.display()}" }
        }
    }
}

#[component]
fn FileItem(entry: FileEntry, index: usize, library: Signal<Library>) -> Element {
    rsx! {
        div {
            key: "{entry.name}",
            class: if entry.is_directory { "file-item directory" } else { "file-item file" },
            onclick: move |_| {
                if entry.is_directory {
                    library.write().enter_directory(index);
                } else {
                    library.write().select_file(index);
                }
            },

            span { class: "file-icon",
                if entry.is_directory { "📁" } else { "📄" }
            }
            span { class: "file-name", "{entry.name}" }
        }
    }
}

#[component]
fn EmptyFileList() -> Element {
    rsx! {
        div { class: "empty-state",
            p { "No ebook files found in this directory" }
        }
    }
}

#[component]
fn ErrorBanner(error: String, library: Signal<Library>) -> Element {
    rsx! {
        div { class: "error-banner",
            p { "{error}" }
            button {
                onclick: move |_| library.write().clear_err(),
                "✕"
            }
        }
    }
}
