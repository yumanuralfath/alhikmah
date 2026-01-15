use dioxus::prelude::*;
use hikmah_core::file_picker::{select_ebook_file, select_ebook_files};
use hikmah_core::library::Library;
use hikmah_core::models::BookMetadata;
use hikmah_core::utils::{format_date, format_size};
use tracing::info;

#[component]
pub fn LibraryView(library: Signal<Library>) -> Element {
    let loading = use_signal(|| false);
    let lib = library.read();

    rsx! {
        div { class: "container",
            Header { library, loading }

            if loading() {
                LoadingOverlay {}
            }

            main { class: "library-grid",
                for book in lib.books.iter() {
                    BookCard {
                        book: book.clone(),
                        library
                    }
                }

                if lib.books.is_empty() && !loading() {
                    EmptyLibrary {}
                }
            }

            if let Some(err) = &lib.err {
                ErrorBanner {
                    error: err.clone(),
                    library
                }
            }
        }
    }
}

#[component]
fn Header(library: Signal<Library>, loading: Signal<bool>) -> Element {
    let add_single_book = move |_| {
        spawn({
            let mut library = library;
            let mut loading = loading;

            async move {
                loading.set(true);

                match select_ebook_file().await {
                    Ok(Some(file)) => {
                        info!("File selected: {} ({} bytes)", file.name, file.size);

                        let mut lib = library.write();
                        if let Err(e) = lib.add_book_from_file(file) {
                            lib.err = Some(format!("Failed to add book: {}", e));
                        }
                    }
                    Ok(None) => {
                        info!("File selection cancelled");
                    }
                    Err(e) => {
                        library.write().err = Some(format!("File picker error: {}", e));
                    }
                }

                loading.set(false);
            }
        });
    };

    let add_multiple_books = move |_| {
        spawn({
            let mut library = library;
            let mut loading = loading;

            async move {
                loading.set(true);

                match select_ebook_files().await {
                    Ok(files) => {
                        if !files.is_empty() {
                            info!("Selected {} files", files.len());

                            let errors = {
                                let mut lib = library.write();
                                lib.add_multiple_books(files)
                            };

                            if !errors.is_empty() {
                                library.write().err = Some(format!(
                                    "Some books failed to add:\n{}",
                                    errors.join("\n")
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        library.write().err = Some(format!("File picker error: {}", e));
                    }
                }

                loading.set(false);
            }
        });
    };

    let book_count = library.read().books.len();

    rsx! {
        header {
            div { class: "header-content",
                h1 { "📚 My Ebook Library" }
                if book_count > 0 {
                    p { class: "book-count", "{book_count} books" }
                }
            }

            div { class: "button-group",
                button {
                    class: "btn-add",
                    onclick: add_single_book,
                    disabled: loading(),
                    "➕ Add Book"
                }
                button {
                    class: "btn-add",
                    onclick: add_multiple_books,
                    disabled: loading(),
                    "➕ Add Multiple"
                }
            }
        }
    }
}

#[component]
fn BookCard(book: BookMetadata, library: Signal<Library>) -> Element {
    let book_id = book.id.clone();
    let book_title = book.title.clone();

    let open_book = {
        let book_id = book_id.clone();
        move |_| {
            info!("Opening book: {}", book_id);
            // TODO: Reader view
        }
    };

    let delete_book = move |_| {
        info!("Deleting book: {}", book_title);

        let mut lib = library.write();
        if let Err(e) = lib.remove_book(&book_id) {
            lib.err = Some(format!("Failed to remove: {}", e));
        }
    };

    rsx! {
        div {
            key: "{book.id}",
            class: "book-card",

            div { class: "book-cover",
                span { class: "book-icon", "📖" }
                span { class: "book-format-badge", "{book.format}" }
            }

            div { class: "book-info",
                h3 {
                    class: "book-title",
                    title: "{book.title}",
                    "{book.title}"
                }
                p { class: "author", "by {book.author}" }
                p { class: "size", "📊 {format_size(book.size)}" }
                p { class: "date", "📅 {format_date(&book.added_date)}" }

                if book.last_read_position > 0 {
                    p { class: "progress",
                        "📖 Progress: {book.last_read_position}/{book.total_pages}"
                    }
                }
            }

            div { class: "book-actions",
                button {
                    class: "btn-primary",
                    onclick: open_book,
                    "📖 Read"
                }
                button {
                    class: "btn-danger",
                    onclick: delete_book,
                    "🗑️ Delete"
                }
            }
        }
    }
}

#[component]
fn LoadingOverlay() -> Element {
    rsx! {
        div { class: "loading-overlay",
            div { class: "loading-spinner",
                div { class: "spinner" }
                p { "Loading..." }
            }
        }
    }
}

#[component]
fn EmptyLibrary() -> Element {
    rsx! {
        div { class: "empty-state",
            p { class: "empty-icon", "📚" }
            h2 { "Your library is empty" }
            p { class: "empty-hint",
                "Click 'Add Book' to import your first ebook"
            }
            p { class: "empty-subhint",
                "Supports EPUB, PDF, and TXT files"
            }
        }
    }
}

#[component]
fn ErrorBanner(error: String, library: Signal<Library>) -> Element {
    rsx! {
        div { class: "error-banner",
            span { class: "error-icon", "⚠️" }
            p { class: "error-text", "{error}" }
            button {
                class: "error-close",
                onclick: move |_| library.write().clear_err(),
                "✕"
            }
        }
    }
}
