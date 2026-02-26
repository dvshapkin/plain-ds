use std::ptr;
use crate::core::node_one_link::{Iter, IterMut, Node};
use crate::core::List;
use super::IntoIter;

pub struct OrderedList<T> {
    head: *mut Node<T>, // 8 bytes
    last: *mut Node<T>, // 8 bytes
    size: usize,        // 8 bytes
}

impl<T> OrderedList<T> {
    /// Creates empty ordered list.
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            last: ptr::null_mut(),
            size: 0,
        }
    }
}

impl<'a, T: 'a> List<'a, T> for OrderedList<T> {
    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn head(&self) -> Option<&T> {
        todo!()
    }

    fn last(&self) -> Option<&T> {
        todo!()
    }

    fn get(&self, index: usize) -> crate::Result<&T> {
        todo!()
    }

    fn get_mut(&mut self, index: usize) -> crate::Result<&mut T> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item=&'a T> {
        Iter::new(self.head)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item=&'a mut T> {
        IterMut::new(self.head)
    }

    fn into_iter(self) -> impl Iterator<Item=T> {
        IntoIter::new(self)
    }

    fn push(&mut self, payload: T) {
        todo!()
    }

    fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    fn pop_front(&mut self) -> Option<T> {
        todo!()
    }

    fn remove(&mut self, index: usize) -> crate::Result<T> {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize> {
        todo!()
    }

    fn sort(&mut self) {
        todo!()
    }
}