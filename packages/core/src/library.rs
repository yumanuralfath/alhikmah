use crate::models::{BookFormat, BookMetadata, FileEntry};
use crate::storage::StorageManager;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Library {
    pub books: Vec<BookMetadata>,
    pub current_path: PathBuf,
    pub browsing_mode: bool,
    pub file_list: Vec<FileEntry>,
    pub err: Option<String>,
}

impl Library {
    pub fn new() -> Self {
        let mut lib = Self {
            books: vec![],
            current_path: PathBuf::from("."),
            browsing_mode: false,
            file_list: vec![],
            err: None,
        };

        if let Err(e) = lib.load_library() {
            log::error!("Failed to load library: {}", e);
        }

        lib
    }

    // ===== PERSISTENCE =====

    pub fn load_library(&mut self) -> Result<(), String> {
        let library_path = StorageManager::get_library_path()?;

        if !library_path.exists() {
            if let Some(parent) = library_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
            return Ok(());
        }

        let content = fs::read_to_string(&library_path)
            .map_err(|e| format!("Failed to read library: {}", e))?;

        let books: Vec<BookMetadata> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse library: {}", e))?;

        self.books = books;
        Ok(())
    }

    pub fn save_library(&self) -> Result<(), String> {
        let library_path = StorageManager::get_library_path()?;

        if let Some(parent) = library_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(&self.books)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        fs::write(&library_path, content).map_err(|e| format!("Failed to write library: {}", e))?;

        Ok(())
    }

    // ===== FILE BROWSING =====

    pub fn start_browsing(&mut self) {
        self.browsing_mode = true;
        self.current_path = StorageManager::get_default_browse_path();
        self.reload_file_list();
    }

    pub fn reload_file_list(&mut self) {
        log::info!("Loading files from: {:?}", self.current_path);

        let paths = match fs::read_dir(&self.current_path) {
            Ok(p) => p,
            Err(e) => {
                self.err = Some(format!("Cannot read directory: {}", e));
                return;
            }
        };

        self.file_list.clear();
        self.err = None;

        for entry in paths.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let is_directory = path.is_dir();

            if is_directory || Self::is_ebook_file(&path) {
                self.file_list.push(FileEntry {
                    name,
                    path,
                    is_directory,
                });
            }
        }

        self.file_list
            .sort_by(|a, b| match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

        log::info!("Loaded {} items", self.file_list.len());
    }

    fn is_ebook_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "epub" | "pdf" | "txt")
        } else {
            false
        }
    }

    pub fn go_up(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.reload_file_list();
        }
    }

    pub fn enter_directory(&mut self, index: usize) {
        if let Some(entry) = self.file_list.get(index) {
            if entry.is_directory {
                self.current_path = entry.path.clone();
                self.reload_file_list();
            }
        }
    }

    pub fn select_file(&mut self, index: usize) {
        if let Some(entry) = self.file_list.get(index) {
            if !entry.is_directory {
                if let Err(e) = self.add_book_from_path(entry.path.clone()) {
                    self.err = Some(e);
                } else {
                    self.browsing_mode = false;
                }
            }
        }
    }

    pub fn cancel_browsing(&mut self) {
        self.browsing_mode = false;
        self.file_list.clear();
    }

    // ===== BOOK MANAGEMENT =====

    fn add_book_from_path(&mut self, source_path: PathBuf) -> Result<(), String> {
        let storage_path = StorageManager::get_storage_path()?;
        fs::create_dir_all(&storage_path)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        let format = Self::detect_format(&source_path)?;

        let file_ext = source_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("epub");
        let unique_name = format!("{}.{}", uuid::Uuid::new_v4(), file_ext);
        let dest_path = storage_path.join(&unique_name);

        fs::copy(&source_path, &dest_path).map_err(|e| format!("Failed to copy file: {}", e))?;

        let metadata = Self::parse_metadata(&dest_path, &unique_name, format)?;

        self.books.push(metadata);
        self.save_library()?;

        log::info!("Book added successfully");
        Ok(())
    }

    pub fn remove_book(&mut self, book_id: &str) -> Result<(), String> {
        if let Some(book) = self.books.iter().find(|b| b.id == book_id) {
            let file_path = PathBuf::from(&book.file_path);
            if file_path.exists() {
                fs::remove_file(&file_path).map_err(|e| format!("Failed to delete file: {}", e))?;
            }
        }

        self.books.retain(|b| b.id != book_id);
        self.save_library()?;

        Ok(())
    }

    fn detect_format(path: &Path) -> Result<BookFormat, String> {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or("No file extension")?
            .to_lowercase();

        match ext.as_str() {
            "epub" => Ok(BookFormat::EPUB),
            "pdf" => Ok(BookFormat::PDF),
            "txt" => Ok(BookFormat::TXT),
            _ => Err(format!("Unsupported format: {}", ext)),
        }
    }

    fn parse_metadata(
        path: &Path,
        file_name: &str,
        format: BookFormat,
    ) -> Result<BookMetadata, String> {
        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Ok(BookMetadata {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            author: "Unknown Author".to_string(),
            file_path: path.to_string_lossy().to_string(),
            file_name: file_name.to_string(),
            format,
            cover_image: None,
            last_read_position: 0,
            total_pages: 0,
            added_date: chrono::Utc::now().to_rfc3339(),
        })
    }

    pub fn clear_err(&mut self) {
        self.err = None;
    }
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}
