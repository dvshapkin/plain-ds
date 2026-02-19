//! Single-linked list implementation.

use std::ptr;

#[derive(PartialEq, Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    next: Option<Box<Node<T>>>, // 8 bytes
    payload: T,                 // size_of::<T>() bytes
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(payload: T) -> Self {
        Self {
            next: None,
            payload,
        }
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }
}

pub struct List<T>
where
    T: PartialEq,
{
    head: Option<Box<Node<T>>>, // 8 bytes
    last: *mut Node<T>,         // 8 bytes
    size: usize,                // 8 bytes
}

impl<T> List<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self {
            head: None,
            last: ptr::null_mut(),
            size: 0,
        }
    }

    /// Returns list size.
    /// Efficiency: O(1)
    pub fn len(&self) -> usize {
        self.size
    }

    /// Checks if the list is empty.
    /// Efficiency: O(1)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the fist node of the list.
    /// Efficiency: O(1)
    pub fn head(&self) -> Option<&Node<T>> {
        self.head.as_ref().map(|node| &**node)
    }

    /// Returns the last node of the list.
    /// Efficiency: O(1)
    pub fn last(&self) -> Option<&Node<T>> {
        if self.last.is_null() {
            None
        } else {
            Some(unsafe { &*self.last })
        }
}

    /// Adds a new node to the end of the list.
    /// Efficiency: O(1)
    pub fn push_back(&mut self, payload: T) {
        let mut new_node = Box::new(Node::new(payload));
        let new_node_ptr: *mut Node<T> = &mut *new_node;

        if self.is_empty() {
            self.head = Some(new_node);
        } else {
            unsafe { (*self.last).next = Some(new_node) };
        }
        self.last = new_node_ptr;
        self.size += 1;
    }

    /// Adds a new node to the front of the list.
    /// Efficiency: O(1)
    pub fn push_front(&mut self, payload: T) {
        let mut new_node = Box::new(Node::new(payload));
        let new_node_ptr: *mut Node<T> = &mut *new_node;
        if self.is_empty() {
            self.last = new_node_ptr;
        } else {
            new_node.next = self.head.take();
        }
        self.head = Some(new_node);
        self.size += 1;
    }

    /// Removes a node from the end of the list and returns it.
    /// Efficiency: O(1)
    pub fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    /// Removes a node from the front of the list and returns it.
    /// Efficiency: O(1)
    pub fn pop_front(&mut self) -> Option<T> {
        todo!()
    }

    /// Insert a new node at the specified location in the list.
    /// Efficiency: O(n)
    pub fn insert(&mut self, index: usize, payload: T) {
        todo!()
    }

    /// Removes a node from the specified location in the list.
    /// Efficiency: O(n)
    pub fn remove(&mut self, index: usize) -> T {
        todo!()
    }

    /// Finds the first node whose payload is equal to the given one and returns its index.
    /// Returns `None` if there is no such node.
    /// Efficiency: O(n)
    pub fn find(&self, payload: T) -> Option<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let list: List<u8> = List::new();
        assert_eq!(list.len(), 0, "not zero length after creation");
        assert_eq!(list.head(), None, "not empty head after creation");
        assert_eq!(list.last(), None, "not empty last after creation");
        assert!(list.is_empty(), "is_empty() returns `false` after creation");

        let list: List<String> = List::new();
        assert!(list.is_empty(), "is_empty() returns `false` after creation");

        let list: List<&[char]> = List::new();
        assert!(list.is_empty(), "is_empty() returns `false` after creation");
    }

    #[test]
    fn test_push_back() {
        let mut list: List<u8> = List::new();
        assert!(list.is_empty(), "is_empty() returns `false` after creation");

        list.push_back(1);
        assert_eq!(list.len(), 1, "bad length after push_back()");
        assert_eq!(list.head(), Some(&Node::new(1)), "incorrect head after push_back()");
        assert_eq!(list.last(), Some(&Node::new(1)), "incorrect last after push_back()");
        assert!(!list.is_empty(), "is_empty() returns `true` after push_back()");

        list.push_back(2);
        assert_eq!(list.len(), 2, "bad length after push_back()");
        assert!(list.head().is_some(), "head is None after push_back()");
        assert_eq!(list.head().unwrap().payload, 1, "incorrect head payload");
        assert_eq!(list.head().unwrap().next, Some(Box::new(Node::new(2))), "incorrect head.next after push_back()");
        assert_eq!(list.last(), Some(&Node::new(2)), "incorrect last after push_back()");
        assert!(!list.is_empty(), "is_empty() returns `true` after push_back()");

        let mut list: List<String> = List::new();
        list.push_back("hello".to_string());
        assert_eq!(list.len(), 1, "bad length after push_back()");
        assert!(list.head().is_some(), "head is None after push_back()");
        assert_eq!(list.head().unwrap().payload, "hello".to_string(), "incorrect head payload");

        let mut list: List<&[char]> = List::new();
        list.push_back(&['a', 'b', 'c']);
        assert_eq!(list.len(), 1, "bad length after push_back()");
        assert!(list.head().is_some(), "head is None after push_back()");
        assert_eq!(list.head().unwrap().payload, &['a', 'b', 'c'], "incorrect head payload");
    }

    #[test]
    fn test_push_front() {
        let mut list: List<u8> = List::new();
        assert!(list.is_empty(), "is_empty() returns `false` after creation");

        list.push_front(1);
        assert_eq!(list.len(), 1, "bad length after push_front()");
        assert_eq!(list.head(), Some(&Node::new(1)), "incorrect head after push_front()");
        assert_eq!(list.last(), Some(&Node::new(1)), "incorrect last after push_front()");
        assert!(!list.is_empty(), "is_empty() returns `true` after push_front()");

        list.push_front(2);
        assert_eq!(list.len(), 2, "bad length after push_front()");
        assert!(list.head().is_some(), "head is None after push_front()");
        assert_eq!(list.head().unwrap().payload, 2, "incorrect head payload");
        assert_eq!(list.head().unwrap().next, Some(Box::new(Node::new(1))), "incorrect head.next after push_front()");
        assert_eq!(list.last(), Some(&Node::new(1)), "incorrect last after push_front()");
        assert!(!list.is_empty(), "is_empty() returns `true` after push_front()");

        let mut list: List<String> = List::new();
        list.push_front("hello".to_string());
        assert_eq!(list.len(), 1, "bad length after push_front()");
        assert!(list.head().is_some(), "head is None after push_front()");
        assert_eq!(list.head().unwrap().payload, "hello".to_string(), "incorrect head payload");

        let mut list: List<&[char]> = List::new();
        list.push_front(&['a', 'b', 'c']);
        assert_eq!(list.len(), 1, "bad length after push_front()");
        assert!(list.head().is_some(), "head is None after push_front()");
        assert_eq!(list.head().unwrap().payload, &['a', 'b', 'c'], "incorrect head payload");
    }

    #[test]
    fn test_memory_leaks() {
        use drop_tracker::DropTracker;

        let mut tracker = DropTracker::new();

        let mut list = List::new();
        for i in 0..100 {
            list.push_back(tracker.track(i));
        }
        for i in 100..111 {
            list.push_front(tracker.track(i));
        }

        assert_eq!(tracker.alive().count(), 111);

        drop(list);

        assert_eq!(tracker.alive().count(), 0);
        assert_eq!(tracker.dropped().count(), 111);
    }
}
