#[derive(Debug, PartialEq)]
pub enum ListError {
    IndexOutOfBounds { index: usize, len: usize },
}

pub type Result<T> = std::result::Result<T, ListError>;