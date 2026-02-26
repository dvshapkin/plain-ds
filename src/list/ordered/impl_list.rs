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

    /// Collect list values into a vector.
    ///
    /// Efficiency: O(n)
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.state.to_vec()
    }
}

impl<'a, T: 'a> List<'a, T> for OrderedList<T> {
    /// Returns list size.
    ///
    /// Efficiency: O(1)
    fn len(&self) -> usize {
        self.state.len()
    }

    /// Returns the payload value of the first node in the list.
    ///
    /// Efficiency: O(1)
    fn head(&self) -> Option<&T> {
        self.state.head()
    }

    /// Returns the payload value of the last node in the list.
    ///
    /// Efficiency: O(1)
    fn last(&self) -> Option<&T> {
        self.state.last()
    }

    /// Returns an iterator over the immutable items of the list.
    fn iter(&self) -> impl Iterator<Item = &'a T> {
        self.state.iter()
    }

    /// Returns an iterator over the mutable items of the list.
    fn iter_mut(&mut self) -> impl Iterator<Item = &'a mut T> {
        self.state.iter_mut()
    }

    /// Returns an iterator that consumes the list.
    fn into_iter(self) -> impl Iterator<Item = T> {
        IntoIter::new(self)
    }

    /// Adds a new node to the list according to the sort order.
    ///
    /// Efficiency: O(n)
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

    /// Removes a node from the end of the list and returns its payload value.
    ///
    /// Efficiency: O(n)
    fn pop_back(&mut self) -> Option<T> {
        self.state.pop_back()
    }

    /// Removes a node from the front of the list and returns its payload value.
    ///
    /// Efficiency: O(1)
    fn pop_front(&mut self) -> Option<T> {
        self.state.pop_front()
    }

    /// Removes a node from the specified location in the list.
    /// Error returns, if the index out of bounds.
    ///
    /// Efficiency: O(n)
    fn remove(&mut self, index: usize) -> crate::Result<T> {
        self.state.remove(index)
    }

    /// Finds the first node whose payload satisfies the predicate and returns its index.
    /// Returns `None` if there is no such node.
    ///
    /// Efficiency: O(???)
    fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize> {
        todo!()
    }
}
