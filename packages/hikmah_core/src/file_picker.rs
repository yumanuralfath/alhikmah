use dioxus::document::{eval, EvalError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileSelection {
    pub name: String,
    pub r#type: String,
    pub size: u64,
    pub data: String, // base64 data URL
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FilePickerOptions {
    accept: Option<String>,
    capture: Option<String>,
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
        if (attrs.multiple) {
            dioxus.send([]);
        } else {
            dioxus.send(null);
        }
        return;
    }
    const readFile = (file) => new Promise((resolve) => {
        const base = {
            name: file.name,
            type: file.type,
            size: file.size,
        };
        if (attrs.encoding === undefined || attrs.encoding === null) {
            resolve({
                ...base,
                data: null,
            });
            return;
        }
        const reader = new FileReader();
        reader.onload = () => {
            resolve({
                ...base,
                data: reader.result,
            });
        };
        switch (attrs.encoding) {
            case "text":
                reader.readAsText(file);
                break;
            case "data_url":
                reader.readAsDataURL(file);
                break;
            default:
                console.error("Unsupported encoding:", attrs.encoding);
                throw new Error("Unsupported encoding");
        }
    });
    const readFiles = await Promise.all([...files].map(readFile));
    if (attrs.multiple) {
        dioxus.send(readFiles);
    } else {
        dioxus.send(readFiles[0]);
    }
};
input.click();"#;

/// Select a single ebook file (EPUB, PDF, TXT) with base64 data
pub async fn select_ebook_file() -> Result<Option<FileSelection>, EvalError> {
    let options = FilePickerOptions {
        accept: Some(".epub,.pdf,.txt,application/epub+zip,application/pdf,text/plain".to_string()),
        capture: None,
    };

    let mut eval = eval(SELECT_FILE_SCRIPT);
    eval.send(&FilePickerOptionsInternal {
        accept: &options.accept,
        multiple: false,
        capture: &options.capture,
        encoding: Some(DataEncoding::DataUrl),
    })?;

    let data = eval.recv().await?;
    Ok(data)
}

/// Select multiple ebook files with base64 data
pub async fn select_ebook_files() -> Result<Vec<FileSelection>, EvalError> {
    let options = FilePickerOptions {
        accept: Some(".epub,.pdf,.txt,application/epub+zip,application/pdf,text/plain".to_string()),
        capture: None,
    };

    let mut eval = eval(SELECT_FILE_SCRIPT);
    eval.send(&FilePickerOptionsInternal {
        accept: &options.accept,
        multiple: true,
        capture: &options.capture,
        encoding: Some(DataEncoding::DataUrl),
    })?;

    let data = eval.recv().await?;
    Ok(data)
}

/// Detect book format from MIME type or filename
pub fn detect_book_format(mime_type: &str, filename: &str) -> Option<String> {
    match mime_type {
        "application/epub+zip" => Some("epub".to_string()),
        "application/pdf" => Some("pdf".to_string()),
        "text/plain" => Some("txt".to_string()),
        _ => {
            if let Some(ext) = filename.split('.').next_back() {
                let ext_lower = ext.to_lowercase();
                match ext_lower.as_str() {
                    "epub" | "pdf" | "txt" => Some(ext_lower),
                    _ => None,
                }
            } else {
                None
            }
        }
    }
}
