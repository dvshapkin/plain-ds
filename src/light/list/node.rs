use std::ptr;

#[derive(PartialEq, Debug)]
pub struct Node<T> {
    pub next: *mut Node<T>, // 8 bytes
    pub payload: T,         // size_of::<T>() bytes
}

impl<T> Node<T> {
    pub fn new(payload: T) -> Self {
        Self {
            next: ptr::null_mut(),
            payload,
        }
    }
}
