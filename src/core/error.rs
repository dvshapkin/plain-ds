#[derive(Debug, PartialEq)]
pub enum DSError {
    IndexOutOfBounds { index: usize, len: usize },
}

pub type Result<T> = std::result::Result<T, DSError>;