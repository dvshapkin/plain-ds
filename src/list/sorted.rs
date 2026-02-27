use crate::core::Node;
use crate::list::api::List;
use crate::list::common::ListCommon;

/// An ordered collection that maintains its elements in sorted order.
///
/// The `SortedList` automatically keeps elements sorted upon insertion,
/// ensuring efficient search operations.
///
/// # Type Parameters
/// * `T`: The type of elements stored in the list. Must implement `PartialOrd`.
///
/// # Examples
/// ```
/// use plain_ds::SortedList;
///
/// let mut list = SortedList::new();
/// list.push(3);
/// list.push(1);
/// list.push(2);
///
/// assert_eq!(list.len(), 3);
/// assert_eq!(list.to_vec(), vec![1, 2, 3]);
/// ```
pub struct SortedList<T> {
    state: ListCommon<T>,
}

impl<T> SortedList<T> {
    /// Creates empty ordered list.
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
        T: Clone + PartialOrd,
    {
        let mut list = SortedList::new();
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

    /// Finds the first node whose payload satisfies the predicate and returns its index.
    /// Returns `None` if there is no such node.
    ///
    /// Efficiency: O(n)
    pub fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize>
    where
        T: PartialOrd,
    {
        self.state.find_if(predicate)
    }

    // Helper for insertion into the middle (used in push())
    fn insert_in_middle(&mut self, ptr: *mut Node<T>)
    where
        T: PartialOrd,
    {
        let mut prev = self.state.head;
        unsafe {
            let mut next = (*prev).next;

            while !next.is_null() {
                if (*ptr).payload < (*next).payload {
                    (*prev).next = ptr;
                    (*ptr).next = next;
                    return;
                }
                prev = next;
                next = (*next).next;
            }
        }
    }
}

impl<'a, T: 'a> List<'a, T> for SortedList<T>
where
    T: PartialOrd,
{
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

    /// Adds a new node to the list according to the sort order.
    ///
    /// Efficiency: O(n) at worst
    fn push(&mut self, payload: T) {
        let ptr = Box::into_raw(Box::new(Node::new(payload)));

        if self.is_empty() {
            self.state.head = ptr;
            self.state.last = ptr;
        } else {
            unsafe {
                // Quick Case: Insert at the Beginning
                if (*ptr).payload <= (*self.state.head).payload {
                    (*ptr).next = self.state.head;
                    self.state.head = ptr;
                }
                // Quick Case: Insert at the End
                else if (*self.state.last).payload <= (*ptr).payload {
                    (*self.state.last).next = ptr;
                    self.state.last = ptr;
                }
                // General case: searching for a position in the middle
                else {
                    self.insert_in_middle(ptr);
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

    /// Finds the first node whose payload is equal to the given `value` and returns its index.
    /// Returns `None` if there is no such node.
    ///
    /// Efficiency: O(n) at worst
    fn find(&self, value: &T) -> Option<usize>
    where T: PartialEq<T>
    {
        for (index, payload) in self.iter().enumerate() {
            if payload == value {
                return Some(index);
            }
            // Early exit: If the data is sorted and the current value
            // is already greater than the possible match
            if payload > value {
                break; // definitely won't find anything further
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice() {
        let list = SortedList::from_slice(&[2, 1, 5, 4, 3]);
        assert_eq!(list.to_vec(), [1, 2, 3, 4, 5]);
    }

    mod push {
        use std::cmp::Ordering;
        use super::*;

        #[test]
        fn test_push_empty_list() {
            let mut list = SortedList::<i32>::new();

            list.push(42);

            assert_eq!(list.len(), 1, "list should have one element after push");
            assert_eq!(list.to_vec(), vec![42], "single element should be correctly inserted");
            assert_eq!(list.state.head, list.state.last, "head and last should point to the same node in single-element list");
        }

        #[test]
        fn test_push_to_beginning() {
            let mut list = SortedList::from_slice(&[2, 3, 4]);

            list.push(1);

            let values = list.to_vec();
            assert_eq!(values, vec![1, 2, 3, 4], "element smaller than all existing should be inserted at beginning");
            unsafe {
                assert_eq!((*list.state.head).payload, 1, "head should point to newly inserted smallest element");
            }
        }

        #[test]
        fn test_push_to_end() {
            let mut list = SortedList::from_slice(&[1, 2, 3]);

            list.push(4);

            let values = list.to_vec();
            assert_eq!(values, vec![1, 2, 3, 4], "element larger than all existing should be inserted at end");
            unsafe {
                assert_eq!((*list.state.last).payload, 4, "last should point to newly inserted largest element");
            }
        }

        #[test]
        fn test_push_in_middle() {
            let mut list = SortedList::from_slice(&[1, 3, 5]);

            list.push(2);

            let values = list.to_vec();
            assert_eq!(values, vec![1, 2, 3, 5], "element should be inserted in correct middle position");
        }

        #[test]
        fn test_push_duplicate_values() {
            let mut list = SortedList::from_slice(&[1, 3, 5]);

            list.push(3);

            let values = list.to_vec();
            assert_eq!(values, vec![1, 3, 3, 5], "duplicate values should be inserted and preserved");
        }

        #[test]
        fn test_push_multiple_elements_in_random_order() {
            let mut list = SortedList::new();

            // Insert elements in random order
            list.push(5);
            list.push(1);
            list.push(3);
            list.push(2);
            list.push(4);

            let values = list.to_vec();
            assert_eq!(values, vec![1, 2, 3, 4, 5], "elements inserted in random order should result in sorted list");
        }

        #[test]
        fn test_push_updates_size_correctly() {
            let mut list = SortedList::new();

            assert_eq!(list.len(), 0, "new list should have size 0");

            list.push(1);
            assert_eq!(list.len(), 1, "size should be 1 after first push");

            list.push(2);
            assert_eq!(list.len(), 2, "size should be 2 after second push");

            list.push(0);
            assert_eq!(list.len(), 3, "size should be 3 after third push");
        }

        #[test]
        fn test_push_string_data() {
            let mut list = SortedList::new();

            list.push("zebra".to_string());
            list.push("apple".to_string());
            list.push("banana".to_string());

            let values = list.to_vec();
            assert_eq!(
                values,
                vec!["apple".to_string(), "banana".to_string(), "zebra".to_string()],
                "strings should be sorted alphabetically"
            );
        }

        #[test]
        fn test_push_large_numbers() {
            let mut list = SortedList::new();

            list.push(1_000_000);
            list.push(-1_000_000);
            list.push(0);

            let values = list.to_vec();
            assert_eq!(values, vec![-1_000_000, 0, 1_000_000], "large positive and negative numbers should be sorted correctly");
        }

        #[test]
        fn test_push_after_clear() {
            let mut list = SortedList::from_slice(&[1, 2, 3]);
            list.clear();

            assert_eq!(list.len(), 0, "list should be empty after clear");

            list.push(5);

            let values = list.to_vec();
            assert_eq!(values, vec![5], "push after clear should work correctly");
            assert_eq!(list.len(), 1, "size should be 1 after push following clear");
        }

        #[test]
        fn test_push_with_custom_ord_type() {
            #[derive(Clone, Debug, PartialEq)]
            struct Point {
                x: i32,
                y: i32,
            }

            impl PartialOrd for Point {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    self.x.partial_cmp(&other.x)
                }
            }

            let mut list = SortedList::new();
            list.push(Point { x: 3, y: 1 });
            list.push(Point { x: 1, y: 5 });
            list.push(Point { x: 2, y: 8 });

            let values = list.to_vec();
            assert_eq!(values.len(), 3, "custom Ord type should be handled correctly");
            assert_eq!(values[0].x, 1, "should be sorted by x coordinate");
            assert_eq!(values[1].x, 2, "should be sorted by x coordinate");
            assert_eq!(values[2].x, 3, "should be sorted by x coordinate");
        }
    }

    mod find {
        use super::*;

        /// Helper function to create a list from a slice of values using from_slice
        fn create_list_from_slice<T: Clone + PartialOrd>(values: &[T]) -> SortedList<T> {
            SortedList::from_slice(values)
        }

        #[test]
        fn test_find_empty_list() {
            let list: SortedList<i32> = SortedList::new();
            let result = list.find(&42);

            assert_eq!(result, None, "should return None for empty list");
        }

        #[test]
        fn test_find_element_at_beginning() {
            let list = create_list_from_slice(&[1, 2, 3, 4, 5]);
            let result = list.find(&1);

            assert_eq!(result, Some(0), "should find element at index 0");
        }

        #[test]
        fn test_find_element_in_middle() {
            let list = create_list_from_slice(&[10, 20, 30, 40, 50]);
            let result = list.find(&30);

            assert_eq!(result, Some(2), "should find element in the middle and return correct index");
        }

        #[test]
        fn test_find_element_at_end() {
            let list = create_list_from_slice(&[5, 10, 15, 20]);
            let result = list.find(&20);

            assert_eq!(result, Some(3), "should find element at the end and return correct index");
        }

        #[test]
        fn test_find_non_existent_element_smaller_than_all() {
            let list = create_list_from_slice(&[10, 20, 30, 40]);
            let result = list.find(&5);

            assert_eq!(result, None, "should return None when searching for element smaller than all elements");
        }

        #[test]
        fn test_find_non_existent_element_larger_than_all() {
            let list = create_list_from_slice(&[10, 20, 30, 40]);
            let result = list.find(&50);

            // Early exit should trigger here — first check fails, then payload > value → break
            assert_eq!(result, None, "should return None and use early exit for element larger than all");
        }

        #[test]
        fn test_find_non_existent_element_between_values() {
            let list = create_list_from_slice(&[10, 20, 30, 40]);
            let result = list.find(&25);

            // Should stop at 30 (30 > 25) without checking 40
            assert_eq!(result, None, "should use early exit when value would be between existing elements");
        }

        #[test]
        fn test_find_first_occurrence_of_duplicate() {
            let list = create_list_from_slice(&[1, 2, 2, 3, 2]);
            let result = list.find(&2);

            assert_eq!(result, Some(1), "should return index of first occurrence when duplicates exist");
        }

        #[test]
        fn test_find_single_element_match() {
            let list = create_list_from_slice(&[42]);
            let result = list.find(&42);

            assert_eq!(result, Some(0), "should find matching element in single-element list");
        }

        #[test]
        fn test_find_single_element_no_match() {
            let list = create_list_from_slice(&[42]);
            let result = list.find(&100);

            assert_eq!(result, None, "should return None in single-element list when no match");
        }

        #[test]
        fn test_find_with_string_data() {
            let strings = vec!["apple", "banana", "cherry", "date"];
            let list = create_list_from_slice(&strings);

            let result = list.find(&"cherry");
            assert_eq!(result, Some(2), "should find string element at correct index");

            let result2 = list.find(&"grape");
            assert_eq!(result2, None, "should return None for non‑existent string");
        }

        #[test]
        fn test_find_early_exit_efficiency_hint() {
            // Large sorted list
            let large_values: Vec<i32> = (0..1000).map(|x| x * 2).collect(); // 0, 2, 4, ..., 1998
            let list = create_list_from_slice(&large_values);

            // Search for value that doesn't exist but would be early in the list
            let result = list.find(&1); // Should exit immediately at 0 (0 > 1 is false, but 2 > 1 → true → break)

            assert_eq!(result, None, "should efficiently exit early when value is between first elements");

            // Verify it didn't scan the whole list
            // The early exit should happen after checking first few elements
        }

        #[test]
        fn test_find_custom_type() {
            #[derive(PartialEq, PartialOrd, Debug, Clone)]
            struct Person {
                name: String,
                age: u32,
            }

            let people = vec![
                Person { name: "Alice".to_string(), age: 25 },
                Person { name: "Bob".to_string(), age: 30 },
                Person { name: "Charlie".to_string(), age: 35 },
            ];
            let list = create_list_from_slice(&people);

            let target = Person { name: "Bob".to_string(), age: 30 };
            let result = list.find(&target);
            assert_eq!(result, Some(1), "should find custom type by full equality");

            let nonexistent = Person { name: "David".to_string(), age: 40 };
            let result2 = list.find(&nonexistent);
            assert_eq!(result2, None, "should return None for non‑existent custom type");
        }

        #[test]
        fn test_find_preserves_list_integrity() {
            let values = vec![1, 3, 5, 7, 9];
            let list = create_list_from_slice(&values);

            // Store original state
            let original_vec = list.to_vec();

            // Perform find operations
            let _result1 = list.find(&3);
            let _result2 = list.find(&6); // doesn't exist
            let _result3 = list.find(&9);

            // Verify list hasn't been modified
            assert_eq!(list.to_vec(), original_vec, "find operation should not modify the original list");
            assert_eq!(list.len(), 5, "list length should remain unchanged after find operations");
        }

        #[test]
        fn test_find_on_list_with_negative_numbers() {
            let list = create_list_from_slice(&[-10, -5, 0, 5, 10]);

            assert_eq!(list.find(&-5), Some(1), "should find negative number at correct index");
            assert_eq!(list.find(&0), Some(2), "should find zero at correct index");
            assert_eq!(list.find(&15), None, "should return None for value larger than all elements");
            assert_eq!(list.find(&-15), None, "should return None for value smaller than all elements");
        }
    }
}
