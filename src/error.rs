#[derive(Debug)]
pub enum SwayIOError {
    FileOpenError { file_name: String },
    PathNotFound { path: String },
    FileFormatIncorrect { incorrect: String },
    UnknownDirective { line: String },
}
