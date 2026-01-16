use dioxus::prelude::*;
use hikmah_core::file_picker::select_ebook_files;
use hikmah_core::library::Library;
use hikmah_core::models::BookMetadata;
use hikmah_core::utils::{format_date, format_size};
use tracing::info;

/* =========================
   LIBRARY VIEW (ROOT)
========================= */

#[component]
pub fn LibraryView(library: Signal<Library>) -> Element {
    let loading = use_signal(|| false);
    let lib = library.read();

    rsx! {
        div { class: "min-h-screen bg-base-100",
            Header { library, loading }

            if loading() {
                LoadingOverlay {}
            }

            main {
                class: "p-6 grid gap-6 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3",

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

/* =========================
   HEADER / NAVBAR
========================= */

#[component]
fn Header(library: Signal<Library>, loading: Signal<bool>) -> Element {
    let add_books = move |_| {
        spawn({
            let mut library = library;
            let mut loading = loading;

            async move {
                loading.set(true);

                match select_ebook_files().await {
                    Ok(files) => {
                        if files.is_empty() {
                            loading.set(false);
                            return;
                        }

                        info!("Selected {} files", files.len());

                        let errors = library.write().add_multiple_books(files);
                        if !errors.is_empty() {
                            library.write().err = Some(errors.join("\n"));
                        }
                    }
                    Err(e) => {
                        library.write().err = Some(format!("File picker error: {e}"));
                    }
                }

                loading.set(false);
            }
        });
    };

    let book_count = library.read().books.len();

    rsx! {
        div { class: "navbar bg-base-100 shadow",
            div { class: "flex-1",
                a { class: "btn btn-ghost text-xl", "📚 Bayt Alhikmah" }
                if book_count > 0 {
                    span { class: "ml-2 badge badge-neutral", "{book_count} books" }
                }
            }

            div { class: "flex-none",
                button {
                    class: "btn btn-primary",
                    onclick: add_books,
                    disabled: loading(),
                    "➕ Add Book"
                }
            }
        }
    }
}

/* =========================
   BOOK CARD
========================= */

#[component]
fn BookCard(book: BookMetadata, library: Signal<Library>) -> Element {
    let book_id = book.id.clone();

    let delete_book = move |_| {
        let mut lib = library.write();
        if let Err(e) = lib.remove_book(&book_id) {
            lib.err = Some(format!("Failed to remove: {e}"));
        }
    };

    rsx! {
        div {
            key: "{book.id}",
            class: "card bg-base-100 shadow hover:shadow-lg transition",

            div { class: "card-body",
                div { class: "flex items-center justify-between",
                    h2 { class: "card-title truncate", "{book.title}" }
                    span { class: "badge badge-outline", "{book.format}" }
                }

                p { class: "text-sm opacity-70", "by {book.author}" }
                p { "📊 {format_size(book.size)}" }
                p { "📅 {format_date(&book.added_date)}" }

                if book.last_read_position > 0 {
                    progress {
                        class: "progress progress-primary mt-2",
                        value: "{book.last_read_position}",
                        max: "{book.total_pages}"
                    }
                }

                div { class: "card-actions justify-end mt-4",
                    button { class: "btn btn-sm btn-primary", "📖 Read" }
                    button {
                        class: "btn btn-sm btn-error",
                        onclick: delete_book,
                        "🗑️ Delete"
                    }
                }
            }
        }
    }
}

/* =========================
   LOADING OVERLAY
========================= */

#[component]
fn LoadingOverlay() -> Element {
    rsx! {
        div {
            class: "fixed inset-0 bg-black/40 flex items-center justify-center z-50",

            div { class: "card bg-base-100 p-6 items-center",
                span { class: "loading loading-spinner loading-lg text-primary" }
                p { class: "mt-2", "Loading..." }
            }
        }
    }
}

/* =========================
   EMPTY STATE
========================= */

#[component]
fn EmptyLibrary() -> Element {
    rsx! {
        div { class: "col-span-full",
            div { class: "hero py-24",
                div { class: "hero-content text-center",
                    div {
                        h1 { class: "text-6xl", "📚" }
                        h2 { class: "text-2xl font-bold mt-4", "Your library is empty" }
                        p { class: "opacity-70 mt-2",
                            "Click 'Add Book' to import your first ebook"
                        }
                        p { class: "text-sm opacity-50",
                            "Supports EPUB, PDF, and TXT"
                        }
                    }
                }
            }
        }
    }
}

/* =========================
   ERROR BANNER
========================= */

#[component]
fn ErrorBanner(error: String, library: Signal<Library>) -> Element {
    rsx! {
        div { class: "fixed bottom-4 right-4 z-50 max-w-md",
            div { class: "alert alert-error shadow-lg",
                span { "⚠️ {error}" }
                button {
                    class: "btn btn-sm btn-ghost",
                    onclick: move |_| library.write().clear_err(),
                    "✕"
                }
            }
        }
    }
}
