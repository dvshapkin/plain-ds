use std::ptr;

use super::IntoIter;
use crate::core::List;
use crate::core::node_one_link::Node;
use crate::list::common::ListCommon;

type Comparator<T> = fn(&T, &T) -> bool;

pub struct OrderedList<T> {
    state: ListCommon<T>,
    compare: Comparator<T>,
}

impl<T: PartialOrd> OrderedList<T> {
    /// Creates empty ordered list.
    pub fn new() -> Self {
        Self {
            state: ListCommon::new(),
            compare: |lhs: &T, rhs: &T| lhs < rhs,
        }
    }

    /// Creates list from slice.
    pub fn from_slice(slice: &mut [T]) -> Self
    where
        T: Clone + Ord,
    {
        let mut list = OrderedList::new();
        for value in slice.iter() {
            list.push((*value).clone());
        }
        list
    }
}

impl<'a, T: 'a> List<'a, T> for OrderedList<T> {
    fn len(&self) -> usize {
        self.state.len()
    }

    fn head(&self) -> Option<&T> {
        todo!()
    }

    fn last(&self) -> Option<&T> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = &'a T> {
        self.state.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &'a mut T> {
        self.state.iter_mut()
    }

    fn into_iter(self) -> impl Iterator<Item = T> {
        IntoIter::new(self)
    }

    fn push(&mut self, payload: T) {
        let ptr = Box::into_raw(Box::new(Node::new(payload)));
        if self.is_empty() {
            self.state.head = ptr;
            self.state.last = ptr;
        } else {
            // Finding the insert point
            let mut next = self.state.head;
            let mut prev: *mut Node<T> = ptr::null_mut();
            let mut done = false;
            unsafe {
                while !next.is_null() {
                    if (self.compare)(&(*ptr).payload, &(*next).payload) {
                        if !prev.is_null() {
                            (*prev).next = ptr;
                        }
                        (*ptr).next = next;
                        done = true;
                        break;
                    }
                    prev = next;
                    next = (*next).next
                }
                if !done {
                    (*prev).next = ptr;
                    self.state.last = ptr;
                }
            }
        }
        self.state.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        self.state.pop_back()
    }

    fn pop_front(&mut self) -> Option<T> {
        self.state.pop_front()
    }

    fn remove(&mut self, index: usize) -> crate::Result<T> {
        self.state.remove(index)
    }

    fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize> {
        todo!()
    }
}
