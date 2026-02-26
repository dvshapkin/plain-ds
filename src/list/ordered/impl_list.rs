use std::ptr;

use super::IntoIter;
use crate::core::List;
use crate::core::node_one_link::{Iter, IterMut, Node};
use crate::list::common;

type Comparator<T> = fn(&T, &T) -> bool;

pub struct OrderedList<T> {
    head: *mut Node<T>, // 8 bytes
    last: *mut Node<T>, // 8 bytes
    size: usize,        // 8 bytes
    compare: Comparator<T>,
}

impl<T: PartialOrd> OrderedList<T> {
    /// Creates empty ordered list.
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            last: ptr::null_mut(),
            size: 0,
            compare: |lhs: &T, rhs: &T| lhs < rhs,
        }
    }

    /// Creates list from slice.
    pub fn from_slice(slice: &mut [T]) -> Self
    where
        T: Clone + Ord,
    {
        let mut list = OrderedList::new();
        // if !slice.is_sorted() {
        //     slice.sort();
        // }
        for value in slice.iter() {
            list.push((*value).clone());
        }
        list
    }

    // CAUTION: Use only if it will not cause disruption to the order.
    // fn push_back(&mut self, payload: T) {
    //     common::push_back(self, self.head, self.last, payload);
    //     self.size += 1;
    // }
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

    fn iter(&self) -> impl Iterator<Item = &'a T> {
        Iter::new(self.head)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &'a mut T> {
        IterMut::new(self.head)
    }

    fn into_iter(self) -> impl Iterator<Item = T> {
        IntoIter::new(self)
    }

    fn push(&mut self, payload: T) {
        let ptr = Box::into_raw(Box::new(Node::new(payload)));
        if self.is_empty() {
            self.head = ptr;
            self.last = ptr;
        } else {
            // Finding the insert point
            let mut next = self.head;
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
                    self.last = ptr;
                }
            }
        }
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let old_head = unsafe { Box::from_raw(self.head) };
        self.head = old_head.next;
        if self.len() == 1 {
            self.last = ptr::null_mut();
        }

        self.size -= 1;
        Some(old_head.payload)
    }

    fn remove(&mut self, index: usize) -> crate::Result<T> {
        todo!()
    }

    fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize> {
        todo!()
    }

    fn sort(&mut self) {
        todo!()
    }
}
