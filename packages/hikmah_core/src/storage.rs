use std::path::PathBuf;

pub struct StorageManager;

impl StorageManager {
    pub fn get_storage_path() -> Result<PathBuf, String> {
        dirs::data_local_dir()
            .ok_or("Failed to get local data directory".to_string())
            .map(|p| p.join("EbookReader").join("ebooks"))
    }

    /// Get library metadata file path
    pub fn get_library_path() -> Result<PathBuf, String> {
        dirs::data_local_dir()
            .ok_or("Failed to get local data directory".to_string())
            .map(|p| p.join("EbookReader").join("library.json"))
    }

    /// Get default browsing directory based on platform
    pub fn get_default_browse_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home
        } else {
            PathBuf::from(".")
        }
    }
}
