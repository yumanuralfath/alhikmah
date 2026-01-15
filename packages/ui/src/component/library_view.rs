use dioxus::prelude::*;
use hikmah_core::library::Library;
use hikmah_core::models::BookMetadata;

#[component]
pub fn LibraryView(library: Signal<Library>) -> Element {
    let lib = library.read();

    rsx! {
        div { class: "container",
            header {
                h1 { "📚 My Ebook Library" }
                button {
                    class: "btn-add",
                    onclick: move |_| library.write().start_browsing(),
                    "➕ Add Book"
                }
            }

            main { class: "library-grid",
                for book in lib.books.iter() {
                    BookCard { book: book.clone(), library }
                }

                if lib.books.is_empty() {
                    EmptyLibrary {}
                }
            }
        }
    }
}

#[component]
fn BookCard(book: BookMetadata, library: Signal<Library>) -> Element {
    let book_id = book.id.clone();
    let title = book.title.clone();
    let author = book.author.clone();
    let format = book.format.clone();
    let last_read_position = book.last_read_position;
    let total_pages = book.total_pages;

    rsx! {
        div {
            key: "{book_id}",
            class: "book-card",

            div { class: "book-cover", "📖" }

            div { class: "book-info",
                h3 { "{title}" }
                p { class: "author", "by {author}" }
                p { class: "format", "{format}" }

                if last_read_position > 0 {
                    p { class: "progress",
                        "Progress: {last_read_position}/{total_pages}"
                    }
                }
            }

            div { class: "book-actions",
                button {
                    onclick: {
                        let book_id = book_id.clone();
                        move |_| {
                            tracing::info!("Opening book: {}", book_id);
                        }
                    },
                    "📖 Read"
                }
                button {
                    class: "btn-danger",
                    onclick: {
                        let book_id = book_id.clone();
                        move |_| {
                            if let Err(e) = library.write().remove_book(&book_id) {
                                tracing::error!("Failed to remove book: {}", e);
                            }
                        }
                    },
                    "🗑️ Delete"
                }
            }
        }
    }
}

#[component]
fn EmptyLibrary() -> Element {
    rsx! {
        div { class: "empty-state",
            p { "📚 Your library is empty" }
            p { "Click 'Add Book' to get started!" }
        }
    }
}
