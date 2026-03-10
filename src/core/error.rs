use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum DSError {
    IndexOutOfBounds { index: usize, len: usize },
    EmptyPath,
    WrongPath { path: PathBuf },
    NotAbsolutePath { path: PathBuf },
    NotFile { path: PathBuf },
    NotDirectory { path: PathBuf },
    PathNotFound { path: PathBuf },
}

pub type Result<T> = std::result::Result<T, DSError>;