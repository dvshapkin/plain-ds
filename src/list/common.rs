use std::ptr;
use crate::core::node_one_link::{Iter, IterMut, Node};
use crate::DSError;

pub struct ListCommon<T> {
    pub head: *mut Node<T>, // 8 bytes
    pub last: *mut Node<T>, // 8 bytes
    pub size: usize,        // 8 bytes
}

impl<'a, T: 'a> ListCommon<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            last: ptr::null_mut(),
            size: 0,
        }
    }

    /// Collect list values into a vector.
    ///
    /// Efficiency: O(n)
    #[inline]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut vec = Vec::with_capacity(self.len());
        vec.extend(self.iter().cloned());
        vec
    }

    /// Returns list size.
    ///
    /// Efficiency: O(1)
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns the payload value of the first node in the list.
    ///
    /// Efficiency: O(1)
    #[inline]
    pub fn head(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            Some(unsafe { &(*self.head).payload })
        }
    }

    /// Returns the payload value of the last node in the list.
    ///
    /// Efficiency: O(1)
    #[inline]
    pub fn last(&self) -> Option<&T> {
        if self.last.is_null() {
            None
        } else {
            Some(unsafe { &(*self.last).payload })
        }
    }

    /// Returns an iterator over the immutable items of the list.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'a T> {
        Iter::new(self.head)
    }

    /// Returns an iterator over the mutable items of the list.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'a mut T> {
        IterMut::new(self.head)
    }

    /// Adds a new node to the end of the list.
    ///
    /// Efficiency: O(1)
    #[inline]
    pub fn push_back(&mut self, payload: T) {
        let ptr = Box::into_raw(Box::new(Node::new(payload)));
        if self.len() == 0 {
            self.head = ptr;
        } else {
            unsafe { (*self.last).next = ptr };
        }
        self.last = ptr;
        self.size += 1;
    }

    /// Removes a node from the end of the list and returns its payload value.
    ///
    /// Efficiency: O(n)
    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        // Case: only one node in list
        if self.head == self.last {
            let payload = unsafe { Box::from_raw(self.head).payload };
            self.head = ptr::null_mut();
            self.last = ptr::null_mut();
            self.size -= 1;
            return Some(payload);
        }

        // Finding the penultimate node
        let mut current = self.head;
        unsafe {
            while (*current).next != self.last {
                current = (*current).next;
            }
        }

        // current now points to the penultimate node
        let old_last = self.last;
        self.last = current;
        unsafe { (*self.last).next = ptr::null_mut() };

        // Release the last node and extract the payload
        let payload = unsafe {
            let boxed = Box::from_raw(old_last);
            boxed.payload
        };

        self.size -= 1;
        Some(payload)
    }

    /// Removes a node from the front of the list and returns its payload value.
    ///
    /// Efficiency: O(1)
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len() == 0 {
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

    /// Removes a node from the specified location in the list.
    /// Error returns, if the index out of bounds.
    ///
    /// Efficiency: O(n)
    #[inline]
    pub fn remove(&mut self, index: usize) -> crate::Result<T> {
        if index >= self.size {
            return Err(DSError::IndexOutOfBounds {
                index,
                len: self.size,
            });
        }
        if index == 0 {
            // remove first
            return Ok(self.pop_front().unwrap());
        }
        if index + 1 == self.size {
            // remove last
            return Ok(self.pop_back().unwrap());
        }

        // Finding the removing item
        let mut before = self.head;
        let mut index = index;
        unsafe {
            while index > 1 {
                before = (*before).next;
                index -= 1;
            }
        }

        let removed = unsafe { Box::from_raw((*before).next) };
        unsafe { (*before).next = removed.next };

        self.size -= 1;
        Ok(removed.payload)
    }
}

impl<T> Drop for ListCommon<T> {
    fn drop(&mut self) {
        use std::mem::ManuallyDrop;

        let mut current = ManuallyDrop::new(self.head);
        while !current.is_null() {
            unsafe {
                let node = Box::from_raw(ManuallyDrop::take(&mut current));
                current = ManuallyDrop::new(node.next);
            }
        }
    }
}
