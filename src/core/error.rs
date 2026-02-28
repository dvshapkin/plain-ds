#[derive(Debug, PartialEq)]
pub enum DSError {
    IndexOutOfBounds { index: usize, len: usize },
    EmptyPath,
    WrongPath { path: String },
    NotAbsolutePath { path: String },
    NotFile { path: String },
    NotDirectory { path: String },
}

pub type Result<T> = std::result::Result<T, DSError>;