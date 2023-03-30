use std::path::Path;

#[derive(Debug)]
pub enum SwayIOError {
    FileOpenError { file_name: String },
    PathNotFound { path: String },
    FileFormatIncorrect { incorrect: String },
    UnknownDirective { line: String },
}

impl SwayIOError {
    pub fn path_not_found(p: &dyn AsRef<Path>) -> SwayIOError {
        SwayIOError::PathNotFound {
            path: path_to_error_string(p),
        }
    }
}

pub fn path_to_error_string(p: &dyn AsRef<Path>) -> String {
    p.as_ref().to_string_lossy().to_string()
}
