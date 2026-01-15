use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BookMetadata {
    pub id: String,
    pub title: String,
    pub author: String,
    pub file_path: String,
    pub file_name: String,
    pub format: BookFormat,
    pub cover_image: Option<String>,
    pub last_read_position: usize,
    pub total_pages: usize,
    pub added_date: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BookFormat {
    EPUB,
    PDF,
    TXT,
}

impl std::fmt::Display for BookFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookFormat::EPUB => write!(f, "EPUB"),
            BookFormat::PDF => write!(f, "PDF"),
            BookFormat::TXT => write!(f, "TXT"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: std::path::PathBuf,
    pub is_directory: bool,
}
