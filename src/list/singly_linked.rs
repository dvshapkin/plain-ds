//! This module contains singly-linked list implementation.

use std::ptr;

use crate::list::api::List;
use crate::core::{DSError, Result};
use crate::core::{Node, merge_sort};
use crate::list::common::ListCommon;

/// A singly-linked list implementation with efficient insertion at the front and back.
///
/// The `SinglyLinkedList` stores elements in a linear sequence where each element
/// points to the next one. It provides O(1) push and pop_front operations.
///
/// # Type Parameters
/// * `T`: The type of elements stored in the list.
///
///
/// # Examples
/// ```
/// use plain_ds::SinglyLinkedList;
///
/// let mut list = SinglyLinkedList::new();
/// list.push(1);
/// list.push(2);
/// list.push(3);
///
/// assert_eq!(list.pop(), Some(3));
/// assert_eq!(list.len(), 2);
/// ```
pub struct SinglyLinkedList<T> {
    state: ListCommon<T>,
}

impl<T> SinglyLinkedList<T> {
    /// Creates empty singly-linked list.
    pub fn new() -> Self {
        Self {
            state: ListCommon::new(),
        }
    }

    /// Creates list from slice.
    ///
    /// Efficiency: O(n)
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        let mut list = SinglyLinkedList::new();
        for value in slice {
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

    /// Adds a new node to the front of the list.
    ///
    /// Efficiency: O(1)
    fn push_front(&mut self, payload: T) {
        let ptr = Box::into_raw(Box::new(Node::new(payload)));
        if self.is_empty() {
            self.state.last = ptr;
        } else {
            unsafe { (*ptr).next = self.state.head }
        }
        self.state.head = ptr;
        self.state.size += 1;
    }

    /// Insert a new node at the specified location in the list.
    /// Error returns, if the index out of bounds.
    ///
    /// Efficiency: O(n)
    fn insert(&mut self, index: usize, payload: T) -> Result<()> {
        if index > self.state.size {
            return Err(DSError::IndexOutOfBounds {
                index,
                len: self.state.size,
            });
        }
        if index == self.state.size {
            self.push(payload);
            return Ok(());
        }
        if index == 0 {
            self.push_front(payload);
            return Ok(());
        }

        // Finding the insert point
        let mut current = self.state.head;
        let mut index = index;
        unsafe {
            while index > 1 {
                current = (*current).next;
                index -= 1;
            }
        }

        let mut boxed = Box::new(Node::new(payload));
        unsafe {
            boxed.next = (*current).next;
            (*current).next = Box::into_raw(boxed);
        }

        self.state.size += 1;
        Ok(())
    }

    /// Finds the first node whose payload satisfies the predicate and returns its index.
    /// Returns `None` if there is no such node.
    ///
    /// Efficiency: O(n)
    fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize>
    where
        T: PartialEq,
    {
        self.state.find_if(predicate)
    }

    /// Sorts the list in ascending order using merge sort algorithm.
    ///
    /// Efficiency: O(n log n)
    ///
    /// Space complexity: O(log n) due to recursion stack
    fn sort(&mut self)
    where
        T: PartialOrd + Default,
    {
        if self.state.len() <= 1 {
            return; // Already sorted
        }

        // Extract the head and reset the list
        let head = self.state.head;
        self.state.head = ptr::null_mut();
        self.state.last = ptr::null_mut();
        self.state.size = 0;

        // Sort the extracted nodes and get new head
        let sorted_head = merge_sort(head);

        // Reconstruct the list with sorted nodes
        self.rebuild_from_sorted_list(sorted_head);
    }

    /// Rebuilds the list from a sorted list of nodes
    fn rebuild_from_sorted_list(&mut self, head: *mut Node<T>) {
        self.state.head = head;
        self.state.size = 0;

        if head.is_null() {
            self.state.last = std::ptr::null_mut();
            return;
        }

        // Traverse to find the last node and count size
        let mut current = head;
        self.state.size = 1;

        unsafe {
            while !(*current).next.is_null() {
                current = (*current).next;
                self.state.size += 1;
            }
            self.state.last = current;
        }
    }
}

impl<'a, T: 'a> List<'a, T> for SinglyLinkedList<T> {
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
        self.state.into_iter()
    }

    /// Adds a new node to the end of the list.
    ///
    /// Efficiency: O(1)
    fn push(&mut self, payload: T) {
        self.state.push_back(payload);
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
    fn remove(&mut self, index: usize) -> Result<T> {
        self.state.remove(index)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a list with values [0, 1, 2, ..., n-1]
    fn setup_list(n: usize) -> SinglyLinkedList<usize> {
        let mut list = SinglyLinkedList::new();
        for i in 0..n {
            list.push(i);
        }
        list
    }

    #[test]
    fn test_from_slice() {
        let list = SinglyLinkedList::from_slice(&[2, 1, 5, 4, 3]);
        assert_eq!(list.to_vec(), [2, 1, 5, 4, 3], "The order of elements must be preserved");
    }

    mod get {
        use super::*;

        #[test]
        fn test_get_empty_list() {
            let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
            assert!(
                list.get(0).is_err(),
                "get() on empty list should return error"
            );
            assert!(
                list.get(1).is_err(),
                "get() with any index on empty list should return error"
            );
        }

        #[test]
        fn test_get_index_out_of_bounds() {
            let mut list = SinglyLinkedList::new();
            list.push(10);
            list.push(20);
            list.push(30);

            assert!(
                list.get(3).is_err(),
                "get() with index == size should return error (out of bounds)"
            );
            assert!(
                list.get(4).is_err(),
                "get() with index > size should return error"
            );
            assert!(
                list.get(100).is_err(),
                "get() with large out-of-bounds index should return error"
            );
        }

        #[test]
        fn test_get_first_element() {
            let mut list = SinglyLinkedList::new();
            list.push(100);
            list.push(200);
            list.push(300);

            let result = list.get(0).unwrap();
            assert_eq!(*result, 100, "get(0) should return first element (100)");
        }

        #[test]
        fn test_get_last_element() {
            let mut list = SinglyLinkedList::new();
            list.push(100);
            list.push(200);
            list.push(300);

            let result = list.get(2).unwrap(); // index = size - 1
            assert_eq!(
                *result, 300,
                "get(last_index) should return last element (300)"
            );
        }

        #[test]
        fn test_get_middle_element() {
            let mut list = SinglyLinkedList::new();
            list.push(10);
            list.push(20);
            list.push(30);
            list.push(40);
            list.push(50);

            let result = list.get(2).unwrap(); // middle element
            assert_eq!(*result, 30, "get(2) should return middle element (30)");

            let result2 = list.get(1).unwrap();
            assert_eq!(*result2, 20, "get(1) should return second element (20)");
        }

        #[test]
        fn test_get_single_element_list() {
            let mut list = SinglyLinkedList::new();
            list.push(42);

            let result = list.get(0).unwrap();
            assert_eq!(
                *result, 42,
                "get(0) on single-element list should return that element"
            );

            assert!(
                list.get(1).is_err(),
                "get(1) on single-element list should be out of bounds"
            );
        }

        #[test]
        fn test_get_with_complex_types() {
            // Test with String
            let mut string_list = SinglyLinkedList::new();
            string_list.push("apple".to_string());
            string_list.push("banana".to_string());
            string_list.push("cherry".to_string());

            let first = string_list.get(0).unwrap();
            assert_eq!(first, "apple", "get(0) should return 'apple'");

            let last = string_list.get(2).unwrap();
            assert_eq!(last, "cherry", "get(2) should return 'cherry'");

            // Test with Vec
            let mut vec_list = SinglyLinkedList::new();
            vec_list.push(vec![1, 2]);
            vec_list.push(vec![3, 4]);

            let vec_result = vec_list.get(1).unwrap();
            assert_eq!(vec_result, &vec![3, 4], "get(1) should return vec![3, 4]");
        }

        #[test]
        fn test_get_preserves_list_integrity() {
            let mut list = SinglyLinkedList::new();
            list.push(1);
            list.push(2);
            list.push(3);

            // Get element in the middle
            let _ = list.get(1).unwrap();

            // Verify list is unchanged
            assert_eq!(
                list.len(),
                3,
                "list length should remain unchanged after get()"
            );
            assert_eq!(list.head(), Some(&1), "head should remain the same");
            assert_eq!(list.last(), Some(&3), "last should remain the same");

            // Verify we can still get other elements
            assert_eq!(
                *list.get(0).unwrap(),
                1,
                "get(0) after get(1) should still work"
            );
            assert_eq!(
                *list.get(2).unwrap(),
                3,
                "get(2) after get(1) should still work"
            );
        }

        #[test]
        fn test_get_mut_empty_list() {
            let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
            assert!(
                list.get_mut(0).is_err(),
                "get_mut() on empty list should return error"
            );
        }

        #[test]
        fn test_get_mut_index_out_of_bounds() {
            let mut list = SinglyLinkedList::new();
            list.push(10);
            list.push(20);

            assert!(
                list.get_mut(2).is_err(),
                "get_mut() with index == size should return error"
            );
            assert!(
                list.get_mut(5).is_err(),
                "get_mut() with large out-of-bounds index should return error"
            );
        }

        #[test]
        fn test_get_mut_first_element() {
            let mut list = SinglyLinkedList::new();
            list.push(100);
            list.push(200);
            list.push(300);

            let mut_ref = list.get_mut(0).unwrap();
            *mut_ref = 999;

            assert_eq!(
                *list.get(0).unwrap(),
                999,
                "first element should be modified to 999"
            );
            assert_eq!(
                *list.head().unwrap(),
                999,
                "head should reflect the modification"
            );
        }

        #[test]
        fn test_get_mut_last_element() {
            let mut list = SinglyLinkedList::new();
            list.push(100);
            list.push(200);
            list.push(300);

            let mut_ref = list.get_mut(2).unwrap(); // last element
            *mut_ref = 888;

            assert_eq!(
                *list.get(2).unwrap(),
                888,
                "last element should be modified to 888"
            );
            assert_eq!(
                *list.last().unwrap(),
                888,
                "last should reflect the modification"
            );
        }

        #[test]
        fn test_get_mut_middle_element() {
            let mut list = SinglyLinkedList::new();
            list.push(10);
            list.push(20);
            list.push(30);
            list.push(40);

            let mut_ref = list.get_mut(2).unwrap(); // third element
            *mut_ref *= 2; // 30 * 2 = 60

            assert_eq!(
                *list.get(2).unwrap(),
                60,
                "middle element should be doubled to 60"
            );
        }

        #[test]
        fn test_get_mut_single_element_list() {
            let mut list = SinglyLinkedList::new();
            list.push(42);

            let mut_ref = list.get_mut(0).unwrap();
            *mut_ref += 1;

            assert_eq!(
                *list.get(0).unwrap(),
                43,
                "single element should be modified to 43"
            );
        }

        #[test]
        fn test_get_mut_with_complex_types() {
            // Test with String — modify by pushing more text
            let mut string_list = SinglyLinkedList::new();
            string_list.push("hello".to_string());
            string_list.push("world".to_string());

            let mut_str = string_list.get_mut(0).unwrap();
            mut_str.push_str(" there");

            assert_eq!(
                string_list.get(0).unwrap(),
                "hello there",
                "first string should be modified"
            );

            // Test with Vec — modify by adding elements
            let mut vec_list = SinglyLinkedList::new();
            vec_list.push(vec![1, 2]);
            vec_list.push(vec![3, 4]);

            let mut_vec = vec_list.get_mut(1).unwrap();
            mut_vec.push(5);

            assert_eq!(
                vec_list.get(1).unwrap(),
                &vec![3, 4, 5],
                "second vector should have new element"
            );
        }

        #[test]
        fn test_get_mut_preserves_list_integrity() {
            let mut list = SinglyLinkedList::new();
            list.push(1);
            list.push(2);
            list.push(3);

            // Modify middle element
            let mut_ref = list.get_mut(1).unwrap();
            *mut_ref *= 10; // 2 becomes 20

            // Verify list structure is intact
            assert_eq!(
                list.len(),
                3,
                "list length should remain unchanged after get_mut()"
            );
            assert_eq!(list.head(), Some(&1), "head should remain the same");
            assert_eq!(list.last(), Some(&3), "last should remain the same");

            // Verify other elements are accessible and unchanged
            assert_eq!(
                *list.get(0).unwrap(),
                1,
                "first element should be unchanged"
            );
            assert_eq!(*list.get(2).unwrap(), 3, "last element should be unchanged");
        }

        #[test]
        fn test_multiple_get_mut_calls() {
            let mut list = SinglyLinkedList::new();
            list.push(10);
            list.push(20);
            list.push(30);

            // First modification
            let first = list.get_mut(0).unwrap();
            *first += 5; // 10 becomes 15

            // Second modification on different element
            let last = list.get_mut(2).unwrap();
            *last *= 2; // 30 becomes 60

            // Final verification
            let values: Vec<i32> = list.iter().copied().collect();
            assert_eq!(
                values,
                vec![15, 20, 60],
                "all modifications should be applied correctly"
            );
        }

        #[test]
        fn test_get_mut_then_get() {
            let mut list = SinglyLinkedList::new();
            list.push(5);
            list.push(15);
            list.push(25);

            // Modify using get_mut
            let mid = list.get_mut(1).unwrap();
            *mid = 99;

            // Immediately read using get
            let mid_value = list.get(1).unwrap();

            assert_eq!(
                *mid_value, 99,
                "get() should reflect changes made by get_mut()"
            );
        }

        #[test]
        fn test_get_mut_error_propagation() {
            let mut list = SinglyLinkedList::new();
            list.push(1);
            list.push(2);

            // Try to get mutable reference to out-of-bounds index
            let result = list.get_mut(5);
            assert!(
                result.is_err(),
                "get_mut() with out-of-bounds index should return error"
            );

            // Ensure list is still valid after failed operation
            assert_eq!(
                list.len(),
                2,
                "list should remain unchanged after failed get_mut()"
            );
            assert_eq!(*list.get(0).unwrap(), 1, "first element should remain 1");
            assert_eq!(*list.get(1).unwrap(), 2, "second element should remain 2");
        }
    }

    mod push {
        use super::*;

        #[test]
        fn test_push_to_empty_list_updates_head_and_last() {
            let mut list = SinglyLinkedList::new();

            list.push(100);
            assert_eq!(list.len(), 1);
            assert_eq!(list.head(), Some(&100));
            assert_eq!(list.last(), Some(&100));

            let mut list2 = SinglyLinkedList::new();
            list2.push_front(200);
            assert_eq!(list2.len(), 1);
            assert_eq!(list2.head(), Some(&200));
            assert_eq!(list2.last(), Some(&200));
        }

        #[test]
        fn test_push_front() {
            let mut list: SinglyLinkedList<u8> = SinglyLinkedList::new();
            assert_eq!(list.len(), 0, "is_empty() returns `false` after creation");

            list.push_front(1);
            assert_eq!(list.len(), 1, "bad length after push_front()");
            assert_eq!(list.head(), Some(&1), "incorrect head after push_front()");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_front()");
            assert_ne!(
                list.len(), 0,
                "is_empty() returns `true` after push_front()"
            );

            list.push_front(2);
            assert_eq!(list.len(), 2, "bad length after push_front()");
            assert!(list.head().is_some(), "head is None after push_front()");
            assert_eq!(list.head(), Some(&2), "incorrect head payload");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_front()");
            assert_ne!(
                list.len(), 0,
                "is_empty() returns `true` after push_front()"
            );

            let mut list: SinglyLinkedList<String> = SinglyLinkedList::new();
            list.push_front("hello".to_string());
            assert_eq!(list.len(), 1, "bad length after push_front()");
            assert!(list.head().is_some(), "head is None after push_front()");
            assert_eq!(list.head().unwrap(), "hello", "incorrect head payload");

            let mut list: SinglyLinkedList<&[char]> = SinglyLinkedList::new();
            list.push_front(&['a', 'b', 'c']);
            assert_eq!(list.len(), 1, "bad length after push_front()");
            assert!(list.head().is_some(), "head is None after push_front()");
            assert_eq!(
                list.head().unwrap(),
                &['a', 'b', 'c'],
                "incorrect head payload"
            );
        }

        #[test]
        fn test_mix_push() {
            let mut list: SinglyLinkedList<u8> = SinglyLinkedList::new();
            assert_eq!(list.len(), 0, "is_empty() returns `false` after creation");

            list.push(1);
            assert_eq!(list.len(), 1, "bad length after push_back()");
            assert_eq!(list.head(), Some(&1), "incorrect head after push_back()");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_back()");
            assert_ne!(list.len(), 0, "is_empty() returns `true` after push_back()");

            list.push_front(2);
            assert_eq!(list.len(), 2, "bad length after push_front()");
            assert!(list.head().is_some(), "head is None after push_front()");
            assert_eq!(list.head(), Some(&2), "incorrect head payload");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_front()");
            assert_ne!(
                list.len(), 0,
                "is_empty() returns `true` after push_front()"
            );

            list.push(3);
            assert_eq!(list.len(), 3, "bad length after push_back()");
            assert!(list.head().is_some(), "head is None after push_back()");
            assert_eq!(list.head(), Some(&2), "incorrect head payload");
            assert_eq!(list.last(), Some(&3), "incorrect last after push_back()");
            assert_ne!(list.len(), 0, "is_empty() returns `true` after push_back()");
        }
    }

    mod mixed {
        use super::*;

        #[test]
        fn test_mixed_push_pop_operations() {
            let mut list = SinglyLinkedList::new();

            list.push(1);
            list.push_front(0);
            list.push(2);

            // List: [0, 1, 2]
            assert_eq!(list.len(), 3);
            assert_eq!(list.head(), Some(&0));
            assert_eq!(list.last(), Some(&2));

            assert_eq!(list.pop_front(), Some(0));
            assert_eq!(list.pop_back(), Some(2));
            assert_eq!(list.pop_front(), Some(1));
            assert!(list.is_empty());

            // Try one more pop
            assert_eq!(list.pop_back(), None);
            assert_eq!(list.pop_front(), None);
        }

        #[test]
        fn test_size_consistency_after_operations() {
            let mut list = SinglyLinkedList::new();

            // Push back
            list.push(10);
            assert_eq!(list.len(), 1, "size after push(10) should be 1");

            list.push(20);
            assert_eq!(list.len(), 2, "size after second push should be 2");

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
            let mut list = SinglyLinkedList::new();

            // Start: empty
            assert_eq!(list.head(), None);
            assert_eq!(list.last(), None);

            // push(1)
            list.push(1);
            assert_eq!(list.head(), Some(&1));
            assert_eq!(list.last(), Some(&1));

            // push_front(0)
            list.push_front(0);
            assert_eq!(list.head(), Some(&0));
            assert_eq!(list.last(), Some(&1));

            // push(2)
            list.push(2);
            assert_eq!(list.head(), Some(&0));
            assert_eq!(list.last(), Some(&2));

            // pop_front() → removes 0
            list.pop_front();
            assert_eq!(list.head(), Some(&1));
            assert_eq!(list.last(), Some(&2));

            // pop_back() → removes 2
            list.pop_back();
            assert_eq!(list.head(), Some(&1));
            assert_eq!(list.last(), Some(&1));

            // Final pop
            list.pop_front();
            assert_eq!(list.head(), None);
            assert_eq!(list.last(), None);
            assert!(list.is_empty());
        }
    }

    mod complex_types {
        use super::*;

        #[test]
        fn test_complex_types_string() {
            let mut list = SinglyLinkedList::new();
            list.push("hello".to_string());
            list.push("world".to_string());

            assert_eq!(list.len(), 2);
            assert_eq!(list.head().unwrap(), "hello");
            assert_eq!(list.last().unwrap(), "world");

            assert_eq!(list.pop_front().unwrap(), "hello".to_string());
            assert_eq!(list.pop_back().unwrap(), "world".to_string());
            assert!(list.is_empty());
        }

        #[test]
        fn test_complex_types_vec() {
            let mut list = SinglyLinkedList::new();
            list.push(vec![1, 2]);
            list.push(vec![3, 4]);

            assert_eq!(list.len(), 2);
            assert_eq!(list.head().unwrap(), &vec![1, 2]);
            assert_eq!(list.last().unwrap(), &vec![3, 4]);

            let popped_front = list.pop_front().unwrap();
            assert_eq!(popped_front, vec![1, 2]);

            let popped_back = list.pop_back().unwrap();
            assert_eq!(popped_back, vec![3, 4]);
            assert!(list.is_empty());
        }
    }

    mod insert {
        use super::*;

        #[test]
        fn test_insert_at_beginning_empty_list() {
            let mut list = SinglyLinkedList::new();
            assert!(
                list.insert(0, 42).is_ok(),
                "insert at index 0 in empty list should succeed"
            );
            assert_eq!(list.len(), 1, "list size should be 1 after insertion");
            assert_eq!(list.head(), Some(&42), "head should contain inserted value");
            assert_eq!(list.last(), Some(&42), "last should contain inserted value");
        }

        #[test]
        fn test_insert_at_beginning_non_empty_list() {
            let mut list = setup_list(3); // [0, 1, 2]
            assert!(
                list.insert(0, 99).is_ok(),
                "insert at beginning should succeed"
            );
            assert_eq!(list.len(), 4, "size should increase by 1");
            assert_eq!(list.head(), Some(&99), "new head should be 99");
            assert_eq!(list.find(&99), Some(0), "find should locate 99 at index 0");
        }

        #[test]
        fn test_insert_at_end() {
            let mut list = setup_list(2); // [0, 1]
            assert!(
                list.insert(2, 999).is_ok(),
                "insert at end (index == size) should succeed"
            );
            assert_eq!(list.len(), 3, "size should increase by 1");
            assert_eq!(list.last(), Some(&999), "last element should be 999");
            assert_eq!(
                list.find(&999),
                Some(2),
                "find should locate 999 at index 2"
            );
        }

        #[test]
        fn test_insert_in_middle() {
            let mut list = setup_list(3); // [0, 1, 2]
            assert!(
                list.insert(1, 50).is_ok(),
                "insert in middle should succeed"
            );
            assert_eq!(list.len(), 4, "size should increase by 1");

            // Verify the order: [0, 50, 1, 2]
            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&0));
            assert_eq!(iter.next(), Some(&50));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
        }

        #[test]
        fn test_insert_out_of_bounds() {
            let mut list = setup_list(2); // [0, 1]

            // Index greater than size
            assert!(
                list.insert(3, 42).is_err(),
                "insert with index > size should return error"
            );

            // Very large index
            assert!(
                list.insert(100, 42).is_err(),
                "insert with large out-of-bounds index should return error"
            );

            // Empty list with non-zero index
            let mut empty_list = SinglyLinkedList::new();
            assert!(
                empty_list.insert(1, 42).is_err(),
                "insert to empty list with index > 0 should return error"
            );
        }

        #[test]
        fn test_insert_with_complex_types_string() {
            let mut list = SinglyLinkedList::new();
            list.push("first".to_string());
            list.push("third".to_string());

            assert!(
                list.insert(1, "second".to_string()).is_ok(),
                "insert string in middle should succeed"
            );
            assert_eq!(list.len(), 3, "size should be 3 after insertion");

            // Verify order: ["first", "second", "third"]
            let values: Vec<String> = list.iter().map(|payload| payload.clone()).collect();
            assert_eq!(values, vec!["first", "second", "third"]);
        }

        #[test]
        fn test_insert_multiple_times() {
            let mut list = SinglyLinkedList::new();

            // Insert at various positions multiple times
            assert!(list.insert(0, 10).is_ok());
            assert!(list.insert(1, 30).is_ok());
            assert!(list.insert(1, 20).is_ok()); // Insert between 10 and 30

            assert_eq!(list.len(), 3, "final size should be 3");

            // Expected order: [10, 20, 30]
            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&10));
            assert_eq!(iter.next(), Some(&20));
            assert_eq!(iter.next(), Some(&30));
        }

        #[test]
        fn test_insert_preserves_head_and_last_pointers() {
            let mut list = setup_list(2); // [0, 1]

            // Insert in the middle
            assert!(list.insert(1, 5).is_ok());

            // Head should still be the first element
            assert_eq!(list.head(), Some(&0), "head pointer should remain correct");

            // Last should still be the last element
            assert_eq!(list.last(), Some(&1), "last pointer should remain correct");
        }

        #[test]
        fn test_insert_edge_cases() {
            // Test inserting into a list with one element
            let mut single_element = SinglyLinkedList::new();
            single_element.push(100);

            // Insert at beginning (should work)
            assert!(single_element.insert(0, 50).is_ok());
            assert_eq!(single_element.find(&50), Some(0));
            assert_eq!(single_element.find(&100), Some(1));

            // Insert at end (should work)
            assert!(single_element.insert(2, 150).is_ok());
            assert_eq!(single_element.find(&150), Some(2));
        }
    }

    mod clear {
        use super::*;

        #[test]
        fn test_clear_empty_list() {
            let mut list = SinglyLinkedList::<u8>::new();
            assert!(list.is_empty(), "list should be empty initially");

            list.clear();

            assert!(
                list.is_empty(),
                "clear() on empty list should leave it empty"
            );
            assert_eq!(
                list.len(),
                0,
                "length should remain 0 after clear() on empty list"
            );
            assert_eq!(
                list.head(),
                None,
                "head should be None after clear() on empty list"
            );
            assert_eq!(
                list.last(),
                None,
                "last should be None after clear() on empty list"
            );
        }

        #[test]
        fn test_clear_single_element_list() {
            let mut list = SinglyLinkedList::new();
            list.push(42);
            assert!(!list.is_empty(), "list should not be empty before clear()");
            assert_eq!(list.len(), 1, "list should have length 1 before clear()");

            list.clear();

            assert!(list.is_empty(), "list should be empty after clear()");
            assert_eq!(list.len(), 0, "length should be 0 after clear()");
            assert_eq!(list.head(), None, "head should be None after clear()");
            assert_eq!(list.last(), None, "last should be None after clear()");
        }

        #[test]
        fn test_clear_multiple_elements_list() {
            let mut list = SinglyLinkedList::new();
            list.push(10);
            list.push(20);
            list.push(30);
            assert_eq!(list.len(), 3, "list should have 3 elements before clear()");

            list.clear();

            assert!(
                list.is_empty(),
                "list should be empty after clearing multiple elements"
            );
            assert_eq!(
                list.len(),
                0,
                "length should be 0 after clearing multiple elements"
            );
            assert_eq!(
                list.head(),
                None,
                "head should be None after clearing multiple elements"
            );
            assert_eq!(
                list.last(),
                None,
                "last should be None after clearing multiple elements"
            );
        }

        #[test]
        fn test_clear_then_reuse_list() {
            let mut list = SinglyLinkedList::new();
            list.push(1);
            list.push(2);

            list.clear();
            assert!(list.is_empty(), "list should be empty after clear()");

            // Reuse the list after clearing
            list.push(100);
            list.push(200);

            assert_eq!(
                list.len(),
                2,
                "list should accept new elements after clear()"
            );
            assert_eq!(*list.head().unwrap(), 100, "new head should be 100");
            assert_eq!(*list.last().unwrap(), 200, "new last should be 200");
        }

        #[test]
        fn test_clear_with_complex_types() {
            // Test with String
            let mut string_list = SinglyLinkedList::new();
            string_list.push("apple".to_string());
            string_list.push("banana".to_string());

            string_list.clear();
            assert!(
                string_list.is_empty(),
                "string list should be empty after clear()"
            );
            assert_eq!(
                string_list.len(),
                0,
                "string list length should be 0 after clear()"
            );

            // Test with Vec
            let mut vec_list = SinglyLinkedList::new();
            vec_list.push(vec![1, 2]);
            vec_list.push(vec![3, 4]);

            vec_list.clear();
            assert!(
                vec_list.is_empty(),
                "vec list should be empty after clear()"
            );
            assert_eq!(
                vec_list.len(),
                0,
                "vec list length should be 0 after clear()"
            );
        }

        #[test]
        fn test_clear_preserves_list_integrity() {
            let mut list = SinglyLinkedList::new();
            list.push(5);
            list.push(10);
            list.push(15);

            let initial_len = list.len();
            let head_before = list.head().cloned();
            let last_before = list.last().cloned();
            assert_eq!(initial_len, 3);
            assert_eq!(head_before.unwrap(), 5);
            assert_eq!(last_before.unwrap(), 15);

            list.clear();

            // Verify the list is properly cleared
            assert!(list.is_empty(), "list should be empty after clear()");
            assert_eq!(list.len(), 0, "length should be 0 after clear()");
            assert_eq!(list.head(), None, "head should be None after clear()");
            assert_eq!(list.last(), None, "last should be None after clear()");

            // Ensure we can create a new list and it works correctly
            let mut new_list = SinglyLinkedList::new();
            new_list.push(100);
            assert_eq!(
                new_list.len(),
                1,
                "new list should work correctly after previous clear()"
            );
        }

        #[test]
        fn test_clear_performance_consistency() {
            // Test that clear() works correctly regardless of list size
            for size in &[0, 1, 5, 10, 100] {
                let mut list = SinglyLinkedList::new();

                // Fill list with values
                for i in 0..*size {
                    list.push(i);
                }

                assert_eq!(
                    list.len(),
                    *size,
                    "list should have correct length before clear() for size {}",
                    size
                );

                list.clear();

                assert!(
                    list.is_empty(),
                    "list of size {} should be empty after clear()",
                    size
                );
                assert_eq!(
                    list.len(),
                    0,
                    "list of size {} should have length 0 after clear()",
                    size
                );
                assert_eq!(
                    list.head(),
                    None,
                    "head should be None for cleared list of size {}",
                    size
                );
                assert_eq!(
                    list.last(),
                    None,
                    "last should be None for cleared list of size {}",
                    size
                );
            }
        }

        #[test]
        fn test_clear_after_mixed_operations() {
            let mut list = SinglyLinkedList::new();

            // Perform various operations
            list.push(1);
            list.push_front(0);
            list.push(2);
            list.pop_front(); // removes 0
            list.push(3);

            // List should now be [1, 2, 3]
            assert_eq!(
                list.len(),
                3,
                "list should have 3 elements after mixed operations"
            );
            assert_eq!(
                *list.head().unwrap(),
                1,
                "head should be 1 after mixed operations"
            );
            assert_eq!(
                *list.last().unwrap(),
                3,
                "last should be 3 after mixed operations"
            );

            list.clear();

            assert!(
                list.is_empty(),
                "list should be empty after clear() following mixed operations"
            );
            assert_eq!(
                list.len(),
                0,
                "length should be 0 after clear() following mixed operations"
            );
            assert_eq!(
                list.head(),
                None,
                "head should be None after clear() following mixed operations"
            );
            assert_eq!(
                list.last(),
                None,
                "last should be None after clear() following mixed operations"
            );
        }
    }

    mod sort {
        use super::*;

        #[test]
        fn test_sort_empty_list() {
            let mut list = SinglyLinkedList::<i32>::new();
            assert_eq!(list.len(), 0, "list should be empty initially");

            list.sort();

            assert_eq!(list.len(), 0, "empty list should remain empty after sort()");
            assert!(list.is_empty(), "empty list should be empty after sort()");
        }

        #[test]
        fn test_sort_single_element() {
            let mut list = SinglyLinkedList::new();
            list.push(42);
            assert_eq!(list.len(), 1, "list should have one element");

            list.sort();

            assert_eq!(
                list.len(),
                1,
                "single element list should have same length after sort()"
            );
            let values = list.to_vec();
            assert_eq!(values, vec![42], "single element should remain unchanged");
        }

        #[test]
        fn test_sort_already_sorted() {
            let mut list = SinglyLinkedList::from_slice(&[1, 2, 3, 4, 5]);
            assert_eq!(list.len(), 5, "list should have 5 elements");

            list.sort();

            let values = list.to_vec();
            assert_eq!(
                values,
                vec![1, 2, 3, 4, 5],
                "already sorted list should remain sorted"
            );
        }

        #[test]
        fn test_sort_reverse_sorted() {
            let mut list = SinglyLinkedList::from_slice(&[5, 4, 3, 2, 1]);
            assert_eq!(list.len(), 5, "list should have 5 elements");

            list.sort();

            let values = list.to_vec();
            assert_eq!(
                values,
                vec![1, 2, 3, 4, 5],
                "reverse sorted list should become ascending"
            );
        }

        #[test]
        fn test_sort_random_order() {
            let mut list = SinglyLinkedList::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6]);
            assert_eq!(list.len(), 8, "list should have 8 elements");

            list.sort();

            let values = list.to_vec();
            assert_eq!(
                values,
                vec![1, 1, 2, 3, 4, 5, 6, 9],
                "random order list should be sorted correctly"
            );
        }

        #[test]
        fn test_sort_with_duplicates() {
            let mut list = SinglyLinkedList::from_slice(&[2, 2, 1, 1, 3, 3]);
            assert_eq!(list.len(), 6, "list should have 6 elements");

            list.sort();

            let values = list.to_vec();
            assert_eq!(
                values,
                vec![1, 1, 2, 2, 3, 3],
                "list with duplicates should be sorted with duplicates preserved"
            );
        }

        #[test]
        fn test_sort_two_elements_unsorted() {
            let mut list = SinglyLinkedList::from_slice(&[2, 1]);
            assert_eq!(list.len(), 2, "list should have 2 elements");

            list.sort();

            let values = list.to_vec();
            assert_eq!(
                values,
                vec![1, 2],
                "two elements should be sorted in ascending order"
            );
        }

        #[test]
        fn test_sort_two_elements_sorted() {
            let mut list = SinglyLinkedList::from_slice(&[1, 2]);
            assert_eq!(list.len(), 2, "list should have 2 elements");

            list.sort();

            let values = list.to_vec();
            assert_eq!(
                values,
                vec![1, 2],
                "already sorted two elements should remain the same"
            );
        }

        #[test]
        fn test_sort_large_list() {
            // Create a large list with random-like pattern
            let mut data: Vec<i32> = (1..=1000).collect();
            // Shuffle by reversing every 10 elements
            for chunk in data.chunks_mut(10) {
                chunk.reverse();
            }

            let mut list = SinglyLinkedList::new();
            for &value in &data {
                list.push(value);
            }

            assert_eq!(list.len(), 1000, "large list should have 1000 elements");

            list.sort();

            let sorted_data: Vec<i32> = (1..=1000).collect();
            let values = list.to_vec();
            assert_eq!(values, sorted_data, "large list should be sorted correctly");
        }

        #[test]
        fn test_sort_after_operations() {
            let mut list = SinglyLinkedList::new();

            // Perform various operations
            list.push(5);
            list.push_front(1);
            list.push(3);
            list.pop_front(); // removes 1
            list.push(2);

            // List should now be [5, 3, 2]
            assert_eq!(
                list.len(),
                3,
                "list should have 3 elements after mixed operations"
            );

            list.sort();

            let values = list.to_vec();
            assert_eq!(
                values,
                vec![2, 3, 5],
                "list after mixed operations should be sorted correctly"
            );
        }

        #[test]
        fn test_sort_string_list() {
            let mut list = SinglyLinkedList::new();
            list.push("zebra".to_string());
            list.push("apple".to_string());
            list.push("banana".to_string());
            list.push("cherry".to_string());

            assert_eq!(list.len(), 4, "string list should have 4 elements");

            list.sort();

            let values: Vec<String> = list.into_iter().collect();

            assert_eq!(
                values,
                vec![
                    "apple".to_string(),
                    "banana".to_string(),
                    "cherry".to_string(),
                    "zebra".to_string()
                ],
                "string list should be sorted alphabetically"
            );
        }

        #[test]
        fn test_sort_preserves_last_pointer() {
            let mut list = SinglyLinkedList::from_slice(&[3, 1, 4, 2]);

            list.sort();

            // Verify that last pointer is correctly set to the last node
            let last_value = unsafe { (*list.state.last).payload };
            assert_eq!(
                last_value, 4,
                "last pointer should point to the maximum element after sorting"
            );
        }
    }

    mod memory_leaks {
        use super::*;
        use drop_tracker::DropTracker;

        #[test]
        fn test_memory_leaks() {
            let mut tracker = DropTracker::new();

            let mut list = SinglyLinkedList::new();
            for i in 0..100 {
                list.push(tracker.track(i));
            }
            for i in 100..111 {
                list.push_front(tracker.track(i));
            }

            assert_eq!(tracker.alive().count(), 111);

            drop(list);

            assert_eq!(tracker.alive().count(), 0);
            assert_eq!(tracker.dropped().count(), 111);
        }

        #[test]
        fn test_iterators_with_drop_tracker() {
            let mut tracker = DropTracker::new();

            let mut list = SinglyLinkedList::new();
            for i in 0..5 {
                list.push(tracker.track(i));
            }

            assert_eq!(tracker.alive().count(), 5);

            // Use all types of iterators sequentially
            {
                // Итератор по ссылкам
                let ref_count: usize = list.iter().count();
                assert_eq!(ref_count, 5);
            }
            {
                // Mutable iterator (modify all elements)
                for item in list.iter_mut() {
                    **item += 10;
                }
            }
            {
                // IntoIterator — take ownership
                let collected: Vec<_> = list.into_iter().collect();
                assert_eq!(collected, vec![10, 11, 12, 13, 14]);
            }

            // After IntoIterator the list is destroyed
            assert_eq!(tracker.alive().count(), 0);
            assert_eq!(tracker.dropped().count(), 5);
        }

        #[test]
        fn test_memory_leaks_with_remove_operations() {
            let mut tracker = DropTracker::new();

            let mut list = SinglyLinkedList::new();
            for i in 0..50 {
                list.push(tracker.track(i));
            }

            assert_eq!(
                tracker.alive().count(),
                50,
                "50 elements should be alive after push"
            );

            // Removing elements from different positions
            assert_eq!(list.remove(0).unwrap(), 0);
            assert_eq!(list.remove(48).unwrap(), 49); // last item
            assert_eq!(list.remove(24).unwrap(), 25); // middle item

            assert_eq!(
                tracker.alive().count(),
                47,
                "After removing 3 elements, 47 should remain alive"
            );

            // Clear the list completely
            while list.len() > 0 {
                let _ = list.pop_front();
            }

            assert_eq!(
                tracker.alive().count(),
                0,
                "All elements should be dropped after clearing the list"
            );
            assert_eq!(
                tracker.dropped().count(),
                50,
                "All 50 elements should have been dropped"
            );
        }

        #[test]
        fn test_memory_leaks_with_insert_operations() {
            let mut tracker = DropTracker::new();

            let mut list = SinglyLinkedList::new();
            list.push(tracker.track(1));
            list.push(tracker.track(3));

            assert_eq!(
                tracker.alive().count(),
                2,
                "2 elements should be alive initially"
            );

            // Insert an element into the middle
            list.insert(1, tracker.track(2)).unwrap();

            assert_eq!(
                tracker.alive().count(),
                3,
                "3 elements should be alive after insert"
            );

            // Insert at the beginning and end
            list.insert(0, tracker.track(0)).unwrap();
            list.insert(3, tracker.track(4)).unwrap();

            assert_eq!(
                tracker.alive().count(),
                5,
                "5 elements should be alive after all inserts"
            );

            drop(list);

            assert_eq!(
                tracker.alive().count(),
                0,
                "All elements should be dropped when list is dropped"
            );
            assert_eq!(
                tracker.dropped().count(),
                5,
                "All 5 elements should have been dropped"
            );
        }

        #[test]
        fn test_memory_leaks_partial_operations() {
            let mut tracker = DropTracker::new();

            let mut list = SinglyLinkedList::new();
            for i in 0..20 {
                list.push(tracker.track(i));
            }

            assert_eq!(tracker.alive().count(), 20, "20 elements should be alive");

            // Perform several deletion and addition operations
            for _ in 0..5 {
                let _ = list.pop_back();
            }
            for i in 100..103 {
                list.push_front(tracker.track(i));
            }

            assert!(
                tracker.alive().count() < 20,
                "Fewer elements should be alive after partial operations"
            );

            drop(list);

            assert_eq!(
                tracker.alive().count(),
                0,
                "All remaining elements should be dropped"
            );
            assert_eq!(
                tracker.dropped().count(),
                23,
                "Total 23 elements should have been dropped (20 original + 3 added - 5 removed)"
            );
        }

        #[test]
        fn test_memory_leaks_with_complex_types() {
            let mut tracker = DropTracker::new();

            #[derive(Debug, PartialEq, Eq, Hash, Clone)]
            struct ComplexStruct {
                id: usize,
                data: String,
            }

            impl Drop for ComplexStruct {
                fn drop(&mut self) {
                    // Just mark the deletion
                }
            }

            let mut list = SinglyLinkedList::new();
            for i in 0..15 {
                list.push(tracker.track(ComplexStruct {
                    id: i,
                    data: format!("data_{}", i),
                }));
            }

            assert_eq!(
                tracker.alive().count(),
                15,
                "15 ComplexStruct elements should be alive"
            );

            // Deleting multiple elements
            for _ in 0..3 {
                let _ = list.pop_front();
            }

            assert_eq!(
                tracker.alive().count(),
                12,
                "12 ComplexStruct elements should remain alive"
            );

            drop(list);

            assert_eq!(
                tracker.alive().count(),
                0,
                "All ComplexStruct elements should be dropped"
            );
            assert_eq!(
                tracker.dropped().count(),
                15,
                "All 15 ComplexStruct elements should have been dropped"
            );
        }

        #[test]
        fn test_memory_leaks_error_conditions() {
            let mut tracker = DropTracker::new();

            let mut list = SinglyLinkedList::new();
            for i in 0..10 {
                list.push(tracker.track(i));
            }

            assert_eq!(tracker.alive().count(), 10, "10 elements should be alive");

            // Attempted to delete at an invalid index (should not cause leaks)
            assert!(list.remove(15).is_err());

            // Attempt to insert at invalid index
            assert!(list.insert(15, tracker.track(99)).is_err());

            assert_eq!(
                tracker.alive().count(),
                10,
                "10 elements should be alive (10 original + 1 attempted insert)"
            );

            // Clearing the list
            while list.len() > 0 {
                let _ = list.pop_front();
            }

            drop(list); // Explicit deletion

            assert_eq!(
                tracker.alive().count(),
                0,
                "All elements should be dropped even after error conditions"
            );
            assert_eq!(
                tracker.dropped().count(),
                11,
                "All 11 elements should have been dropped"
            );
        }

        // Test that clear() properly frees all nodes and there's no memory leak
        #[test]
        fn test_clear_no_memory_leak_with_drop_tracker() {
            // Create a list with tracked nodes
            let mut list = SinglyLinkedList::new();
            let mut tracker = DropTracker::new();

            // Add several elements — each will be wrapped in DropTracker
            list.push(tracker.track(10));
            list.push(tracker.track(20));
            list.push(tracker.track(30));
            list.push(tracker.track(40));
            list.push(tracker.track(50));

            assert_eq!(list.len(), 5, "list should have 5 elements before clear()");
            assert_eq!(tracker.alive().count(), 5, "no nodes should be dropped yet");

            // Clear the list — all nodes should be freed and their Drop impl called
            list.clear();

            assert!(list.is_empty(), "list should be empty after clear()");
            assert_eq!(list.len(), 0, "length should be 0 after clear()");
            assert_eq!(
                tracker.alive().count(),
                0,
                "all 5 nodes should be dropped during clear()"
            );

            assert_eq!(
                tracker.dropped().count(),
                5,
                "no additional drops should happen after list destruction"
            );
        }

        // Test memory cleanup when clear() is called on a list with complex types
        #[test]
        fn test_clear_complex_types_no_memory_leak() {
            let mut list = SinglyLinkedList::new();
            let mut tracker = DropTracker::new();

            // Use DropTracker with String type
            list.push(tracker.track("first".to_string()));
            list.push(tracker.track("second".to_string()));
            list.push(tracker.track("third".to_string()));

            assert_eq!(list.len(), 3, "complex type list should have correct size");
            assert_eq!(tracker.alive().count(), 3, "no drops before clear()");

            list.clear();

            assert!(
                list.is_empty(),
                "complex type list should be empty after clear()"
            );
            assert_eq!(
                tracker.alive().count(),
                0,
                "all 3 complex nodes should be dropped during clear()"
            );

            assert_eq!(
                tracker.dropped().count(),
                3,
                "no extra drops after complex list destruction"
            );
        }

        // Test that clear() works correctly even if some nodes were already dropped
        // through other operations
        #[test]
        fn test_clear_after_partial_removal_no_leak() {
            let mut list = SinglyLinkedList::new();
            let mut tracker = DropTracker::new();

            // Add 4 elements
            list.push(tracker.track(1));
            list.push(tracker.track(2));
            list.push(tracker.track(3));
            list.push(tracker.track(4));

            assert_eq!(list.len(), 4, "initial list size should be 4");
            assert_eq!(tracker.alive().count(), 4, "no drops at start");

            // Remove two elements manually
            let _ = list.pop_front(); // drops element 1
            let _ = list.pop_back(); // drops element 4

            assert_eq!(list.len(), 2, "list size should be 2 after partial removal");
            assert_eq!(
                tracker.alive().count(),
                2,
                "2 nodes should be dropped by pop_front and pop_back"
            );
            assert_eq!(
                tracker.dropped().count(),
                2,
                "2 nodes should be dropped by pop_front and pop_back"
            );

            // Now clear the remaining two elements
            list.clear();

            assert!(list.is_empty(), "list should be empty after final clear()");
            assert_eq!(
                tracker.alive().count(),
                0,
                "total 4 nodes should be dropped (2 by pop, 2 by clear)"
            );

            assert_eq!(
                tracker.dropped().count(),
                4,
                "no extra drops after list destruction"
            );
        }
    }
}
