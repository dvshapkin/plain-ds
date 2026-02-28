use std::ffi::OsString;

#[derive(Debug, PartialEq)]
pub enum DSError {
    IndexOutOfBounds { index: usize, len: usize },
    EmptyPath,
    NotAbsolutePath { path: OsString },
}

pub type Result<T> = std::result::Result<T, DSError>;