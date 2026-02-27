use crate::core::DSError;
use crate::core::{Iter, IterMut, Node};
use std::ptr;

/// `ListCommon` is a core of all lists implementation.
/// It contains common fields and implement some common methods.
/// This is not for direct usage.
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

    /// Returns an iterator that consumes the list.
    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        IntoIter::new(self)
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

pub struct IntoIter<T> {
    list: ListCommon<T>,
}

impl<T> IntoIter<T> {
    pub fn new(list: ListCommon<T>) -> Self {
        Self { list }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.len() == 0 {
            None
        } else {
            self.list.pop_front()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a list with values [0, 1, 2, ..., n-1]
    fn setup_list(n: usize) -> ListCommon<usize> {
        let mut list = ListCommon::new();
        for i in 0..n {
            list.push_back(i);
        }
        list
    }

    #[test]
    fn test_creation() {
        let list: ListCommon<u8> = ListCommon::new();
        assert_eq!(list.len(), 0, "not zero length after creation");
        assert_eq!(list.head(), None, "not empty head after creation");
        assert_eq!(list.last(), None, "not empty last after creation");
    }

    mod push_back {
        use super::*;

        #[test]
        fn test_push() {
            let mut list: ListCommon<u8> = ListCommon::new();
            assert_eq!(list.len(), 0, "len non zero after creation");

            list.push_back(1);
            assert_eq!(list.len(), 1, "bad length after push_back()");
            assert_eq!(list.head(), Some(&1), "incorrect head after push_back()");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_back()");
            assert_ne!(list.len(), 0, "len() returns 0 after push_back()");

            list.push_back(2);
            assert_eq!(list.len(), 2, "bad length after push_back()");
            assert!(list.head().is_some(), "head is None after push_back()");
            assert_eq!(list.head(), Some(&1), "incorrect head payload");
            assert_eq!(list.last(), Some(&2), "incorrect last after push_back()");
            assert_ne!(!list.len(), 0, "len is zero after push_back()");

            let mut list: ListCommon<String> = ListCommon::new();
            list.push_back("hello".to_string());
            assert_eq!(list.len(), 1, "bad length after push_back()");
            assert!(list.head().is_some(), "head is None after push_back()");
            assert_eq!(list.head().unwrap(), "hello", "incorrect head payload");

            let mut list: ListCommon<&[char]> = ListCommon::new();
            list.push_back(&['a', 'b', 'c']);
            assert_eq!(list.len(), 1, "bad length after push_back()");
            assert!(list.head().is_some(), "head is None after push_back()");
            assert_eq!(
                list.head().unwrap(),
                &['a', 'b', 'c'],
                "incorrect head payload"
            );
        }
    }

    mod pop {
        use super::*;

        #[test]
        fn test_pop_back_empty_list() {
            let mut list: ListCommon<u8> = ListCommon::new();
            assert_eq!(
                list.pop_back(),
                None,
                "pop_back from empty list should return None"
            );
            assert_eq!(
                list.len(),
                0,
                "list should remain empty after pop_back on empty"
            );
        }

        #[test]
        fn test_pop_back_single_element() {
            let mut list = ListCommon::new();
            list.push_back(42);
            assert_eq!(
                list.pop_back(),
                Some(42),
                "pop_back() should return the only element"
            );
            assert_eq!(
                list.len(),
                0,
                "list should be empty after popping the last element"
            );
            assert_eq!(
                list.head(),
                None,
                "head should be None after popping last element"
            );
            assert_eq!(
                list.last(),
                None,
                "last should be None after popping last element"
            );
        }

        #[test]
        fn test_pop_back_multiple_elements() {
            let mut list = setup_list(3); // [0, 1, 2]
            assert_eq!(
                list.pop_back(),
                Some(2),
                "pop_back() should return last element (2)"
            );
            assert_eq!(list.len(), 2, "size should decrease by 1 after pop_back()");
            assert_eq!(list.last(), Some(&1), "new last element should be 1");

            assert_eq!(list.pop_back(), Some(1), "pop_back() should return 1 next");
            assert_eq!(list.len(), 1, "size should be 1 after second pop_back()");
            assert_eq!(list.head(), Some(&0), "head should still be 0");
            assert_eq!(list.last(), Some(&0), "last should now be 0");

            assert_eq!(
                list.pop_back(),
                Some(0),
                "pop_back() should return 0 finally"
            );
            assert_eq!(list.len(), 0, "list should be empty after all pop-backs");
        }

        #[test]
        fn test_pop_front_empty_list() {
            let mut list = ListCommon::<u8>::new();
            assert_eq!(
                list.pop_front(),
                None,
                "pop_front() from empty list should return None"
            );
            assert_eq!(
                list.len(),
                0,
                "list should remain empty after pop_front() on empty"
            );
        }

        #[test]
        fn test_pop_front_single_element() {
            let mut list = ListCommon::new();
            list.push_back(99);
            assert_eq!(
                list.pop_front(),
                Some(99),
                "pop_front() should return the only element"
            );
            assert_eq!(
                list.len(),
                0,
                "list should be empty after popping the only element"
            );
            assert_eq!(list.head(), None, "head should be None after pop");
            assert_eq!(list.last(), None, "last should be None after pop");
        }

        #[test]
        fn test_pop_front_multiple_elements() {
            let mut list = setup_list(3); // [0, 1, 2]
            assert_eq!(
                list.pop_front(),
                Some(0),
                "pop_front should return first element (0)"
            );
            assert_eq!(list.len(), 2, "size should decrease by 1 after pop_front");
            assert_eq!(list.head(), Some(&1), "new head should be 1");
            assert_eq!(list.last(), Some(&2), "last should remain 2");

            assert_eq!(list.pop_front(), Some(1), "pop_front should return 1 next");
            assert_eq!(list.len(), 1, "size should be 1 after second pop_front");
            assert_eq!(list.head(), Some(&2), "head should now be 2");
            assert_eq!(list.last(), Some(&2), "last should also be 2");

            assert_eq!(
                list.pop_front(),
                Some(2),
                "pop_front should return 2 finally"
            );
            assert_eq!(list.len(), 0, "list should be empty after all pop_fronts");
        }
    }

    mod remove {
        use super::*;

        #[test]
        fn test_remove_from_empty_list() {
            let mut list = ListCommon::<u8>::new();
            assert!(
                list.remove(0).is_err(),
                "remove from empty list should return error"
            );
            assert_eq!(list.len(), 0, "size should remain 0");
        }

        #[test]
        fn test_remove_first_element() {
            let mut list = setup_list(3); // [0, 1, 2]
            let removed = list.remove(0).unwrap();
            assert_eq!(removed, 0, "removed value should be 0 (first element)");
            assert_eq!(list.len(), 2, "size should decrease by 1");
            assert_eq!(list.head(), Some(&1), "new head should be 1");
        }

        #[test]
        fn test_remove_last_element() {
            let mut list = setup_list(3); // [0, 1, 2]
            let removed = list.remove(2).unwrap(); // index = size - 1
            assert_eq!(removed, 2, "removed value should be 2 (last element)");
            assert_eq!(list.len(), 2, "size should decrease by 1");
            assert_eq!(list.last(), Some(&1), "new last should be 1");
        }

        #[test]
        fn test_remove_middle_element() {
            let mut list = setup_list(4); // [0, 1, 2, 3]
            let removed = list.remove(1).unwrap(); // remove element at index 1 (value 1)
            assert_eq!(removed, 1, "removed value should be 1");
            assert_eq!(list.len(), 3, "size should decrease by 1");

            // Verify the order: [0, 2, 3]
            let values: Vec<usize> = list.iter().copied().collect();
            assert_eq!(
                values,
                vec![0, 2, 3],
                "list should have correct order after removal"
            );
        }

        #[test]
        fn test_remove_out_of_bounds() {
            let mut list = setup_list(2); // [0, 1]

            // Index equal to size (should be out of bounds)
            assert!(
                list.remove(2).is_err(),
                "remove with index == size should return error"
            );

            // Index greater than size
            assert!(
                list.remove(5).is_err(),
                "remove with large out-of-bounds index should return error"
            );

            // Empty list
            let mut empty_list = ListCommon::<u8>::new();
            assert!(
                empty_list.remove(0).is_err(),
                "remove from empty list should return error"
            );
        }

        #[test]
        fn test_remove_single_element_list() {
            let mut list = ListCommon::new();
            list.push_back(42);
            let removed = list.remove(0).unwrap();
            assert_eq!(removed, 42, "removed value should be 42");
            assert_eq!(
                list.len(),
                0,
                "list should be empty after removing the only element"
            );
            assert_eq!(list.head(), None, "head should be None");
            assert_eq!(list.last(), None, "last should be None");
        }

        #[test]
        fn test_remove_preserves_head_and_last_pointers() {
            let mut list = setup_list(4); // [0, 1, 2, 3]

            // Remove middle element (index 1, value 1)
            let _ = list.remove(1);

            assert_eq!(list.head(), Some(&0), "head pointer should remain correct");
            assert_eq!(list.last(), Some(&3), "last pointer should remain correct");
        }

        #[test]
        fn test_multiple_removes() {
            let mut list = setup_list(5); // [0, 1, 2, 3, 4]

            // Remove second element (index 1, value 1)
            let removed1 = list.remove(1).unwrap();
            assert_eq!(removed1, 1);
            assert_eq!(list.len(), 4);

            // Remove new second element (was 2, now at index 1)
            let removed2 = list.remove(1).unwrap();
            assert_eq!(removed2, 2);
            assert_eq!(list.len(), 3);

            // Final state should be [0, 3, 4]
            let final_values: Vec<usize> = list.iter().copied().collect();
            assert_eq!(
                final_values,
                vec![0, 3, 4],
                "list should have correct values after multiple removes"
            );
        }

        #[test]
        fn test_remove_with_complex_types_string() {
            let mut list = ListCommon::new();
            list.push_back("first".to_string());
            list.push_back("second".to_string());
            list.push_back("third".to_string());

            let removed = list.remove(1).unwrap(); // Remove "second"
            assert_eq!(
                removed,
                "second".to_string(),
                "removed value should be 'second'"
            );
            assert_eq!(list.len(), 2, "size should be 2 after removal");

            // Verify order: ["first", "third"]
            let remaining: Vec<String> = list.iter().map(|s| s.clone()).collect();
            assert_eq!(remaining, vec!["first", "third"]);
        }

        #[test]
        fn test_remove_edge_cases() {
            // Test removing from a list with two elements
            let mut two_elements = ListCommon::new();
            two_elements.push_back(10);
            two_elements.push_back(20);

            // Remove first (index 0)
            let removed_first = two_elements.remove(0).unwrap();
            assert_eq!(removed_first, 10);
            assert_eq!(two_elements.len(), 1);
            assert_eq!(two_elements.head(), Some(&20));

            // Now remove the last (only remaining) element
            let removed_last = two_elements.remove(0).unwrap();
            assert_eq!(removed_last, 20);
            assert_eq!(two_elements.len(), 0);
        }
    }

    mod iterators {
        use super::*;

        #[test]
        fn test_empty_list_iterators() {
            let mut list: ListCommon<i32> = ListCommon::new();

            // Reference iterator
            {
                let mut iter = list.iter();
                assert_eq!(iter.next(), None);
            }

            // Mutable iterator
            {
                let mut iter_mut = list.iter_mut();
                assert_eq!(iter_mut.next(), None);
            }

            // IntoIterator (takes ownership)
            let into_iter = list.into_iter();
            assert_eq!(into_iter.collect::<Vec<_>>(), Vec::<i32>::new());
        }

        #[test]
        fn test_sequential_iteration() {
            let mut list = ListCommon::new();
            for i in 0..5 {
                list.push_back(i);
            }

            // Checking an iterator by references
            let collected: Vec<_> = list.iter().collect();
            assert_eq!(collected, vec![&0, &1, &2, &3, &4]);

            // Checking a mutable iterator (changing values)
            for item in list.iter_mut() {
                *item *= 2;
            }
            let doubled: Vec<_> = list.iter().collect();
            assert_eq!(doubled, vec![&0, &2, &4, &6, &8]);

            // Checking an IntoIterator
            let into_collected: Vec<_> = list.into_iter().collect();
            assert_eq!(into_collected, vec![0, 2, 4, 6, 8]);
        }

        #[test]
        fn test_partial_iteration() {
            let mut list = ListCommon::new();
            for i in 0..10 {
                list.push_back(i);
            }
            {
                let mut iter = list.iter();
                // We only go through the first 3 elements
                assert_eq!(iter.next(), Some(&0));
                assert_eq!(iter.next(), Some(&1));
                assert_eq!(iter.next(), Some(&2));
                // We stop without going all the way
            }
            // The list must remain intact
            assert_eq!(list.len(), 10);

            // You can start the iteration again
            let first_element = list.iter().next();
            assert_eq!(first_element, Some(&0));
        }

        #[test]
        fn test_concurrent_iterators() {
            let mut list = ListCommon::new();
            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            // Creating multiple iterators at once
            let collect1: Vec<_> = list.iter().cloned().collect();
            let collect2: Vec<_> = list.iter().cloned().collect();
            {
                let mut iter3 = list.iter_mut();
                let first_mut = iter3.next().unwrap();
                *first_mut = 100;
            }

            assert_eq!(collect1, vec![1, 2, 3]);
            assert_eq!(collect2, vec![1, 2, 3]);
            assert_eq!(list.iter().collect::<Vec<_>>(), vec![&100, &2, &3]);
        }

        #[test]
        fn test_mutable_iteration_modification() {
            let mut list = ListCommon::new();
            for i in 1..=3 {
                list.push_back(i);
            }

            let mut counter = 0;
            for item in list.iter_mut() {
                counter += 1;
                *item = *item * counter; // Multiply by the iteration number
            }

            let result: Vec<_> = list.iter().copied().collect();
            assert_eq!(result, vec![1, 4, 9]); // 1×1, 2×2, 3×3
        }

        #[test]
        fn test_large_list_iteration() {
            const LARGE_SIZE: usize = 10_000;

            let mut list = ListCommon::new();
            for i in 0..LARGE_SIZE {
                list.push_back(i);
            }

            // Full pass through the iterator
            let sum: usize = list.iter().sum();
            let expected_sum: usize = (0..LARGE_SIZE).sum();
            assert_eq!(sum, expected_sum);

            // Partial iteration with take()
            let first_10: Vec<_> = list.iter().take(10).copied().collect();
            assert_eq!(first_10, (0..10).collect::<Vec<_>>());
        }
    }

    #[cfg(test)]
    mod to_vec {
        use super::*;

        /// Helper function to create a list from a slice of values
        fn create_list_from_slice<T: Clone>(values: &[T]) -> ListCommon<T> {
            let mut list = ListCommon::new();
            for value in values {
                list.push_back(value.clone());
            }
            list
        }

        #[test]
        fn test_to_vec_empty_list() {
            let list: ListCommon<i32> = ListCommon::new();

            let result = list.to_vec();

            assert_eq!(result.len(), 0, "vector from empty list should be empty");
            assert!(result.is_empty(), "result vector should be empty for empty list");
        }

        #[test]
        fn test_to_vec_single_element() {
            let mut list = ListCommon::new();
            list.push_back(42);

            let result = list.to_vec();

            assert_eq!(result.len(), 1, "vector should have one element");
            assert_eq!(result, vec![42], "vector should contain the single element from list");
        }

        #[test]
        fn test_to_vec_multiple_elements() {
            let values = vec![1, 2, 3, 4, 5];
            let list = create_list_from_slice(&values);

            let result = list.to_vec();

            assert_eq!(result.len(), 5, "vector should have 5 elements");
            assert_eq!(result, values, "vector should match original values in order");
        }

        #[test]
        fn test_to_vec_preserves_order() {
            let values = vec!["apple", "banana", "cherry", "date"];
            let list = create_list_from_slice(&values);

            let result = list.to_vec();

            assert_eq!(result, values, "vector should preserve the order of elements from the list");
        }

        #[test]
        fn test_to_vec_with_duplicates() {
            let values = vec![5, 2, 8, 2, 9, 5];
            let list = create_list_from_slice(&values);

            let result = list.to_vec();

            assert_eq!(result, values, "vector should include all duplicates in correct positions");
        }

        #[test]
        fn test_to_vec_capacity_matches_length() {
            let values = vec![10, 20, 30];
            let list = create_list_from_slice(&values);

            let result = list.to_vec();

            assert_eq!(result.capacity(), 3, "vector capacity should match list length due to pre‑allocation");
            assert_eq!(result.len(), 3, "vector length should be 3");
        }

        #[test]
        fn test_to_vec_string_data() {
            let strings = vec![
                "hello".to_string(),
                "world".to_string(),
                "rust".to_string()
            ];
            let list = create_list_from_slice(&strings);

            let result = list.to_vec();

            assert_eq!(result.len(), 3, "vector of strings should have correct length");
            assert_eq!(result, strings, "string vector should match original strings");
        }

        #[test]
        fn test_to_vec_large_list() {
            const SIZE: usize = 1000;
            let values: Vec<usize> = (0..SIZE).collect();
            let list = create_list_from_slice(&values);

            let result = list.to_vec();

            assert_eq!(result.len(), SIZE, "large list should produce vector of correct size");
            for i in 0..SIZE {
                assert_eq!(result[i], i, "element at index {} should be correct", i);
            }
        }

        #[test]
        fn test_to_vec_does_not_modify_original_list() {
            let mut list = ListCommon::new();
            list.push_back(10);
            list.push_back(20);
            list.push_back(30);

            let original_len = list.len();

            let _result = list.to_vec();

            assert_eq!(list.len(), original_len, "to_vec() should not modify the original list");
            assert_eq!(list.head(), Some(&10), "head element should remain unchanged");
            assert_eq!(list.last(), Some(&30), "last element should remain unchanged");
        }

        #[test]
        fn test_to_vec_with_custom_cloneable_type() {
            #[derive(Clone, PartialEq, Debug)]
            struct Point {
                x: i32,
                y: i32,
            }

            let points = vec![
                Point { x: 1, y: 2 },
                Point { x: 3, y: 4 },
                Point { x: 5, y: 6 }
            ];
            let list = create_list_from_slice(&points);

            let result = list.to_vec();

            assert_eq!(result.len(), 3, "vector of custom types should have correct length");
            assert_eq!(result, points, "custom cloneable types should be properly cloned and preserved");
        }
    }
}
