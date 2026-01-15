use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BookMetadata {
    pub id: String,
    pub title: String,
    pub author: String,
    pub file_name: String,
    pub format: BookFormat,
    pub size: u64,
    pub file_data: String, // base64 data URL
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

impl BookFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "epub" => Some(BookFormat::EPUB),
            "pdf" => Some(BookFormat::PDF),
            "txt" => Some(BookFormat::TXT),
            _ => None,
        }
    }

    pub fn to_extension(&self) -> &str {
        match self {
            BookFormat::EPUB => "epub",
            BookFormat::PDF => "pdf",
            BookFormat::TXT => "txt",
        }
    }
}
