use crate::file_picker::{detect_book_format, FileSelection};
use crate::models::{BookFormat, BookMetadata};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct Library {
    pub books: Vec<BookMetadata>,
    pub err: Option<String>,
}

impl Library {
    pub fn new() -> Self {
        let mut lib = Self {
            books: vec![],
            err: None,
        };

        // Load library dari localStorage/browser storage
        if let Err(e) = lib.load_from_browser() {
            error!("Failed to load library: {}", e);
        }

        lib
    }

    // ===== BROWSER STORAGE (IndexedDB/localStorage) =====

    fn load_from_browser(&mut self) -> Result<(), String> {
        // Try to load from browser storage
        // For now, just start empty - browser storage akan di-implement via JavaScript
        info!("Library initialized (browser storage not yet implemented)");
        Ok(())
    }

    fn save_to_browser(&self) -> Result<(), String> {
        // Save to browser storage
        // Will be implemented via JavaScript eval
        info!("Saving library to browser storage");
        Ok(())
    }

    // ===== BOOK MANAGEMENT =====

    pub fn add_book_from_file(&mut self, file: FileSelection) -> Result<(), String> {
        info!("Adding book: {} ({} bytes)", file.name, file.size);

        // Validate file size (max 50MB untuk avoid memory issues)
        const MAX_SIZE: u64 = 50 * 1024 * 1024;
        if file.size > MAX_SIZE {
            return Err(format!(
                "File too large: {} MB (max 50 MB)",
                file.size / (1024 * 1024)
            ));
        }

        // Detect format
        let format_str =
            detect_book_format(&file.r#type, &file.name).ok_or("Unsupported file format")?;

        let format = BookFormat::from_extension(&format_str).ok_or("Invalid book format")?;

        // Extract title from filename
        let title = file
            .name
            .strip_suffix(&format!(".{}", format_str))
            .unwrap_or(&file.name)
            .to_string();

        // Check if book already exists
        if self
            .books
            .iter()
            .any(|b| b.file_name == file.name && b.size == file.size)
        {
            return Err("This book already exists in your library".to_string());
        }

        // Create metadata
        let metadata = BookMetadata {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            author: "Unknown Author".to_string(),
            file_name: file.name,
            format,
            size: file.size,
            file_data: file.data, // Store base64 data directly
            cover_image: None,
            last_read_position: 0,
            total_pages: 0,
            added_date: chrono::Utc::now().to_rfc3339(),
        };

        self.books.push(metadata);
        self.save_to_browser()?;

        info!("Book added successfully!");
        Ok(())
    }

    pub fn add_multiple_books(&mut self, files: Vec<FileSelection>) -> Vec<String> {
        let mut errors = Vec::new();

        for file in files {
            let file_name = file.name.clone();
            if let Err(e) = self.add_book_from_file(file) {
                errors.push(format!("{}: {}", file_name, e));
            }
        }

        errors
    }

    pub fn remove_book(&mut self, book_id: &str) -> Result<(), String> {
        self.books.retain(|b| b.id != book_id);
        self.save_to_browser()?;

        info!("Book removed successfully");
        Ok(())
    }

    pub fn update_reading_position(
        &mut self,
        book_id: &str,
        position: usize,
    ) -> Result<(), String> {
        if let Some(book) = self.books.iter_mut().find(|b| b.id == book_id) {
            book.last_read_position = position;
            self.save_to_browser()?;
        }
        Ok(())
    }

    pub fn search_books(&self, query: &str) -> Vec<&BookMetadata> {
        let query_lower = query.to_lowercase();
        self.books
            .iter()
            .filter(|book| {
                book.title.to_lowercase().contains(&query_lower)
                    || book.author.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn get_book(&self, book_id: &str) -> Option<&BookMetadata> {
        self.books.iter().find(|b| b.id == book_id)
    }

    pub fn clear_err(&mut self) {
        self.err = None;
    }

    pub fn clear_library(&mut self) -> Result<(), String> {
        self.books.clear();
        self.save_to_browser()?;
        info!("Library cleared");
        Ok(())
    }
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}
