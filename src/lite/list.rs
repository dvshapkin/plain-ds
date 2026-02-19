//! Single-linked list implementation.

use std::ptr;

#[derive(PartialEq, Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    next: *mut Node<T>, // 8 bytes
    payload: T,         // size_of::<T>() bytes
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(payload: T) -> Self {
        Self {
            next: ptr::null_mut(),
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
    head: *mut Node<T>, // 8 bytes
    last: *mut Node<T>, // 8 bytes
    size: usize,        // 8 bytes
}

impl<T> List<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
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
        if self.head.is_null() {
            None
        } else {
            Some(unsafe { &*self.head })
        }
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
        let ptr = Box::into_raw(Box::new(Node::new(payload)));
        if self.is_empty() {
            self.head = ptr;
        } else {
            unsafe { (*self.last).next = ptr };
        }
        self.last = ptr;
        self.size += 1;
    }

    /// Adds a new node to the front of the list.
    /// Efficiency: O(1)
    pub fn push_front(&mut self, payload: T) {
        let ptr = Box::into_raw(Box::new(Node::new(payload)));
        if self.is_empty() {
            self.last = ptr;
        } else {
            unsafe { (*ptr).next = self.head }
        }
        self.head = ptr;
        self.size += 1;
    }

    /// Removes a node from the end of the list and returns it.
    /// Efficiency: O(n)
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
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

    /// Removes a node from the front of the list and returns it.
    /// Efficiency: O(1)
    pub fn pop_front(&mut self) -> Option<T> {
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

impl<T> Drop for List<T>
where
    T: PartialEq,
{
    fn drop(&mut self) {
        if !self.is_empty() {
            let mut current = self.head;
            unsafe {
                while !(*current).next.is_null() {
                    let dead = Box::from_raw(current);
                    current = dead.next;
                }
                let _ = Box::from_raw(current);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: create a list with values [1, 2, 3] for reuse
    fn setup_list() -> List<u8> {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list
    }

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
        assert_eq!(
            list.head(),
            Some(&Node::new(1)),
            "incorrect head after push_back()"
        );
        assert_eq!(
            list.last(),
            Some(&Node::new(1)),
            "incorrect last after push_back()"
        );
        assert!(
            !list.is_empty(),
            "is_empty() returns `true` after push_back()"
        );

        list.push_back(2);
        assert_eq!(list.len(), 2, "bad length after push_back()");
        assert!(list.head().is_some(), "head is None after push_back()");
        assert_eq!(list.head().unwrap().payload, 1, "incorrect head payload");
        assert_eq!(
            list.head().unwrap().next,
            list.last,
            "incorrect head.next after push_back()"
        );
        assert_eq!(
            list.last(),
            Some(&Node::new(2)),
            "incorrect last after push_back()"
        );
        assert!(
            !list.is_empty(),
            "is_empty() returns `true` after push_back()"
        );

        let mut list: List<String> = List::new();
        list.push_back("hello".to_string());
        assert_eq!(list.len(), 1, "bad length after push_back()");
        assert!(list.head().is_some(), "head is None after push_back()");
        assert_eq!(
            list.head().unwrap().payload,
            "hello".to_string(),
            "incorrect head payload"
        );

        let mut list: List<&[char]> = List::new();
        list.push_back(&['a', 'b', 'c']);
        assert_eq!(list.len(), 1, "bad length after push_back()");
        assert!(list.head().is_some(), "head is None after push_back()");
        assert_eq!(
            list.head().unwrap().payload,
            &['a', 'b', 'c'],
            "incorrect head payload"
        );
    }

    #[test]
    fn test_push_to_empty_list_updates_head_and_last() {
        let mut list = List::new();

        list.push_back(100);
        assert_eq!(list.len(), 1);
        assert_eq!(list.head().unwrap().payload, 100);
        assert_eq!(list.last().unwrap().payload, 100);

        let mut list2 = List::new();
        list2.push_front(200);
        assert_eq!(list2.len(), 1);
        assert_eq!(list2.head().unwrap().payload, 200);
        assert_eq!(list2.last().unwrap().payload, 200);
    }

    #[test]
    fn test_push_front() {
        let mut list: List<u8> = List::new();
        assert!(list.is_empty(), "is_empty() returns `false` after creation");

        list.push_front(1);
        assert_eq!(list.len(), 1, "bad length after push_front()");
        assert_eq!(
            list.head(),
            Some(&Node::new(1)),
            "incorrect head after push_front()"
        );
        assert_eq!(
            list.last(),
            Some(&Node::new(1)),
            "incorrect last after push_front()"
        );
        assert!(
            !list.is_empty(),
            "is_empty() returns `true` after push_front()"
        );

        list.push_front(2);
        assert_eq!(list.len(), 2, "bad length after push_front()");
        assert!(list.head().is_some(), "head is None after push_front()");
        assert_eq!(list.head().unwrap().payload, 2, "incorrect head payload");
        assert_eq!(
            list.head().unwrap().next,
            list.last,
            "incorrect head.next after push_front()"
        );
        assert_eq!(
            list.last(),
            Some(&Node::new(1)),
            "incorrect last after push_front()"
        );
        assert!(
            !list.is_empty(),
            "is_empty() returns `true` after push_front()"
        );

        let mut list: List<String> = List::new();
        list.push_front("hello".to_string());
        assert_eq!(list.len(), 1, "bad length after push_front()");
        assert!(list.head().is_some(), "head is None after push_front()");
        assert_eq!(
            list.head().unwrap().payload,
            "hello".to_string(),
            "incorrect head payload"
        );

        let mut list: List<&[char]> = List::new();
        list.push_front(&['a', 'b', 'c']);
        assert_eq!(list.len(), 1, "bad length after push_front()");
        assert!(list.head().is_some(), "head is None after push_front()");
        assert_eq!(
            list.head().unwrap().payload,
            &['a', 'b', 'c'],
            "incorrect head payload"
        );
    }

    #[test]
    fn test_mix_push_back_front() {
        let mut list: List<u8> = List::new();
        assert!(list.is_empty(), "is_empty() returns `false` after creation");

        list.push_back(1);
        assert_eq!(list.len(), 1, "bad length after push_back()");
        assert_eq!(
            list.head(),
            Some(&Node::new(1)),
            "incorrect head after push_back()"
        );
        assert!(
            list.head().unwrap().next.is_null(),
            "incorrect head.next after push_back()"
        );
        assert_eq!(
            list.last(),
            Some(&Node::new(1)),
            "incorrect last after push_back()"
        );
        assert!(
            !list.is_empty(),
            "is_empty() returns `true` after push_back()"
        );

        list.push_front(2);
        assert_eq!(list.len(), 2, "bad length after push_front()");
        assert!(list.head().is_some(), "head is None after push_front()");
        assert_eq!(list.head().unwrap().payload, 2, "incorrect head payload");
        assert_eq!(
            list.head().unwrap().next,
            list.last,
            "incorrect head.next after push_front()"
        );
        assert_eq!(
            list.last(),
            Some(&Node::new(1)),
            "incorrect last after push_front()"
        );
        assert!(
            !list.is_empty(),
            "is_empty() returns `true` after push_front()"
        );

        list.push_back(3);
        assert_eq!(list.len(), 3, "bad length after push_back()");
        assert!(list.head().is_some(), "head is None after push_back()");
        assert_eq!(list.head().unwrap().payload, 2, "incorrect head payload");
        // assert_eq!(
        //     list.head().unwrap().next.payload,
        //     1,
        //     "incorrect head.next after push_back()"
        // );
        assert_eq!(
            list.last(),
            Some(&Node::new(3)),
            "incorrect last after push_back()"
        );
        assert!(
            !list.is_empty(),
            "is_empty() returns `true` after push_back()"
        );
    }

    #[test]
    fn test_pop_back_empty_list() {
        let mut list: List<u8> = List::new();
        assert_eq!(
            list.pop_back(),
            None,
            "pop_back from empty list should return None"
        );
        assert!(
            list.is_empty(),
            "list should remain empty after pop_back on empty"
        );
    }

    #[test]
    fn test_pop_back_single_element() {
        let mut list = List::new();
        list.push_back(42);
        assert_eq!(list.pop_back(), Some(42), "pop_back() should return the only element");
        assert!(list.is_empty(), "list should be empty after popping the last element");
        assert_eq!(list.head(), None, "head should be None after popping last element");
        assert_eq!(list.last(), None, "last should be None after popping last element");
    }

    #[test]
    fn test_pop_back_multiple_elements() {
        let mut list = setup_list();
        assert_eq!(list.pop_back(), Some(3), "pop_back() should return last element (3)");
        assert_eq!(list.len(), 2, "size should decrease by 1 after pop_back()");
        assert_eq!(list.last().unwrap().payload, 2, "new last element should be 2");

        assert_eq!(list.pop_back(), Some(2), "pop_back() should return 2 next");
        assert_eq!(list.len(), 1, "size should be 1 after second pop_back()");
        assert_eq!(list.head().unwrap().payload, 1, "head should still be 1");
        assert_eq!(list.last().unwrap().payload, 1, "last should now be 1");

        assert_eq!(list.pop_back(), Some(1), "pop_back() should return 1 finally");
        assert!(list.is_empty(), "list should be empty after all pop-backs");
    }

    #[test]
    fn test_pop_front_empty_list() {
        let mut list = List::<u8>::new();
        assert_eq!(list.pop_front(), None, "pop_front() from empty list should return None");
        assert!(list.is_empty(), "list should remain empty after pop_front() on empty");
    }

    #[test]
    fn test_pop_front_single_element() {
        let mut list = List::new();
        list.push_front(99);
        assert_eq!(list.pop_front(), Some(99), "pop_front() should return the only element");
        assert!(list.is_empty(), "list should be empty after popping the only element");
        assert_eq!(list.head(), None, "head should be None after pop");
        assert_eq!(list.last(), None, "last should be None after pop");
    }

    #[test]
    fn test_pop_front_multiple_elements() {
        let mut list = setup_list(); // [1, 2, 3]
        assert_eq!(list.pop_front(), Some(1), "pop_front should return first element (1)");
        assert_eq!(list.len(), 2, "size should decrease by 1 after pop_front");
        assert_eq!(list.head().unwrap().payload, 2, "new head should be 2");
        assert_eq!(list.last().unwrap().payload, 3, "last should remain 3");

        assert_eq!(list.pop_front(), Some(2), "pop_front should return 2 next");
        assert_eq!(list.len(), 1, "size should be 1 after second pop_front");
        assert_eq!(list.head().unwrap().payload, 3, "head should now be 3");
        assert_eq!(list.last().unwrap().payload, 3, "last should also be 3");

        assert_eq!(list.pop_front(), Some(3), "pop_front should return 3 finally");
        assert!(list.is_empty(), "list should be empty after all pop_fronts");
    }

    #[test]
    fn test_size_consistency_after_operations() {
        let mut list = List::new();

        // Push back
        list.push_back(10);
        assert_eq!(list.len(), 1, "size after push_back(10) should be 1");

        list.push_back(20);
        assert_eq!(list.len(), 2, "size after second push_back should be 2");

        // Pop back
        list.pop_back();
        assert_eq!(list.len(), 1, "size after pop_back should be 1");

        // Push front
        list.push_front(5);
        assert_eq!(list.len(), 2, "size after push_front(5) should be 2");

        // Pop front
        list.pop_front();
        assert_eq!(list.len(), 1, "size after pop_front should be 1");

        // Final pop
        list.pop_back();
        assert_eq!(list.len(), 0, "size should be 0 after all pops");
        assert!(list.is_empty(), "list should be empty");
    }

    #[test]
    fn test_head_last_consistency_after_mixed_operations() {
        let mut list = List::new();

        // Start: empty
        assert_eq!(list.head(), None);
        assert_eq!(list.last(), None);

        // push_back(1)
        list.push_back(1);
        assert_eq!(list.head().unwrap().payload, 1);
        assert_eq!(list.last().unwrap().payload, 1);

        // push_front(0)
        list.push_front(0);
        assert_eq!(list.head().unwrap().payload, 0);
        assert_eq!(list.last().unwrap().payload, 1);

        // push_back(2)
        list.push_back(2);
        assert_eq!(list.head().unwrap().payload, 0);
        assert_eq!(list.last().unwrap().payload, 2);

        // pop_front() → removes 0
        list.pop_front();
        assert_eq!(list.head().unwrap().payload, 1);
        assert_eq!(list.last().unwrap().payload, 2);

        // pop_back() → removes 2
        list.pop_back();
        assert_eq!(list.head().unwrap().payload, 1);
        assert_eq!(list.last().unwrap().payload, 1);

        // Final pop
        list.pop_front();
        assert_eq!(list.head(), None);
        assert_eq!(list.last(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_complex_types_string() {
        let mut list = List::new();
        list.push_back("hello".to_string());
        list.push_back("world".to_string());

        assert_eq!(list.len(), 2);
        assert_eq!(list.head().unwrap().payload, "hello");
        assert_eq!(list.last().unwrap().payload, "world");

        assert_eq!(list.pop_front().unwrap(), "hello".to_string());
        assert_eq!(list.pop_back().unwrap(), "world".to_string());
        assert!(list.is_empty());
    }

    #[test]
    fn test_complex_types_vec() {
        let mut list = List::new();
        list.push_back(vec![1, 2]);
        list.push_back(vec![3, 4]);

        assert_eq!(list.len(), 2);
        assert_eq!(list.head().unwrap().payload, vec![1, 2]);
        assert_eq!(list.last().unwrap().payload, vec![3, 4]);

        let popped_front = list.pop_front().unwrap();
        assert_eq!(popped_front, vec![1, 2]);

        let popped_back = list.pop_back().unwrap();
        assert_eq!(popped_back, vec![3, 4]);
        assert!(list.is_empty());
    }

    #[test]
    fn test_mixed_push_pop_operations() {
        let mut list = List::new();

        list.push_back(1);
        list.push_front(0);
        list.push_back(2);

        // List: [0, 1, 2]
        assert_eq!(list.len(), 3);
        assert_eq!(list.head().unwrap().payload, 0);
        assert_eq!(list.last().unwrap().payload, 2);

        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());

        // Try one more pop
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
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
