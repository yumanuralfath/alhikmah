use dioxus::document::{eval, EvalError};
use serde::{Deserialize, Serialize};

/// =======================
/// Data Model
/// =======================

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileSelection {
    pub name: String,
    pub r#type: String,
    pub size: u64,
    pub data: String, // base64 data URL
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum DataEncoding {
    DataUrl,
}

#[derive(Debug, Serialize)]
struct FilePickerOptionsInternal<'a> {
    accept: &'a Option<String>,
    multiple: bool,
    capture: &'a Option<String>,
    encoding: Option<DataEncoding>,
}

/// =======================
/// JavaScript (Unified)
/// =======================
const SELECT_FILE_SCRIPT: &str = r#"
const attrs = await dioxus.recv();

const input = document.createElement("input");
input.type = "file";

if (attrs.accept) input.accept = attrs.accept;
if (attrs.multiple) input.multiple = true;
if (attrs.capture) input.capture = attrs.capture;

input.onchange = async () => {
    const files = input.files;
    input.remove();

    if (!files || files.length === 0) {
        dioxus.send([]);
        return;
    }

    const readFile = (file) => new Promise((resolve) => {
        const base = {
            name: file.name,
            type: file.type,
            size: file.size,
        };

        if (!attrs.encoding) {
            resolve({ ...base, data: null });
            return;
        }

        const reader = new FileReader();
        reader.onload = () => {
            resolve({ ...base, data: reader.result });
        };

        reader.readAsDataURL(file);
    });

    const results = await Promise.all([...files].map(readFile));
    dioxus.send(results);
};

input.click();
"#;

/// =======================
/// Public API (SINGLE ENTRY)
/// =======================
///
/// Select ebook files (EPUB, PDF, TXT)
/// - Always multi-select
/// - Returns Vec<FileSelection>
/// - User may select one or many files
pub async fn select_ebook_files() -> Result<Vec<FileSelection>, EvalError> {
    let accept =
        Some(".epub,.pdf,.txt,application/epub+zip,application/pdf,text/plain".to_string());

    let mut eval = eval(SELECT_FILE_SCRIPT);
    eval.send(&FilePickerOptionsInternal {
        accept: &accept,
        multiple: true, // unified: always true
        capture: &None,
        encoding: Some(DataEncoding::DataUrl),
    })?;

    eval.recv().await
}

/// =======================
/// Utilities
/// =======================
///
/// Detect book format from MIME type or filename
pub fn detect_book_format(mime_type: &str, filename: &str) -> Option<String> {
    match mime_type {
        "application/epub+zip" => Some("epub".into()),
        "application/pdf" => Some("pdf".into()),
        "text/plain" => Some("txt".into()),
        _ => filename
            .rsplit('.')
            .next()
            .map(|e| e.to_lowercase())
            .filter(|e| matches!(e.as_str(), "epub" | "pdf" | "txt")),
    }
}
