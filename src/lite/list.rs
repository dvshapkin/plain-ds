//! Single-linked list implementation.

use std::ptr;

use anyhow::anyhow;

#[derive(PartialEq, Debug)]
pub struct Node<T> {
    next: *mut Node<T>, // 8 bytes
    payload: T,         // size_of::<T>() bytes
}

impl<T> Node<T> {
    pub fn new(payload: T) -> Self {
        Self {
            next: ptr::null_mut(),
            payload,
        }
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn next(&self) -> Option<&Node<T>> {
        if self.next.is_null() {
            None
        } else {
            Some(unsafe { &*self.next })
        }
    }
}

pub struct List<T> {
    head: *mut Node<T>, // 8 bytes
    last: *mut Node<T>, // 8 bytes
    size: usize,        // 8 bytes
}

impl<T> List<T> {
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

    /// Returns the payload value of the first node in the list.
    /// Efficiency: O(1)
    pub fn head(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            Some(unsafe { &(*self.head).payload })
        }
    }

    /// Returns the payload value of the last node in the list.
    /// Efficiency: O(1)
    pub fn last(&self) -> Option<&T> {
        if self.last.is_null() {
            None
        } else {
            Some(unsafe { &(*self.last).payload })
        }
    }

    /// Returns an iterator over the immutable items of the list.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter {
            current: if self.head.is_null() {
                None
            } else {
                Some(unsafe { &*self.head })
            },
        }
    }

    /// Returns an iterator over the mutable items of the list.
    pub fn iter_mut(&self) -> impl Iterator<Item = &mut T> {
        IterMut {
            current: self.head,
            _marker: Default::default(),
        }
    }

    /// Returns an iterator that consumes the list.
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        IntoIter { list: self }
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

    /// Removes a node from the end of the list and returns its payload value.
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

    /// Removes a node from the front of the list and returns its payload value.
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
    /// Error returns, if the index out of bounds.
    /// Efficiency: O(n)
    pub fn insert(&mut self, index: usize, payload: T) -> anyhow::Result<()> {
        if index > self.size {
            return Err(anyhow!("index out of bounds"));
        }
        if index == self.size {
            self.push_back(payload);
            return Ok(());
        }
        if index == 0 {
            self.push_front(payload);
            return Ok(());
        }

        // Finding the insert point
        let mut current = self.head;
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

        self.size += 1;
        Ok(())
    }

    /// Removes a node from the specified location in the list.
    /// Error returns, if the index out of bounds.
    /// Efficiency: O(n)
    pub fn remove(&mut self, index: usize) -> anyhow::Result<T> {
        if index >= self.size {
            return Err(anyhow!("index out of bounds"));
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

    /// Finds the first node whose payload is equal to the given one and returns its index.
    /// Returns `None` if there is no such node.
    /// Efficiency: O(n)
    pub fn find(&self, value: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        for (index, payload) in self.iter().enumerate() {
            if payload == value {
                return Some(index);
            }
        }
        None
    }
}

impl<T> Drop for List<T> {
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

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            None
        } else {
            let payload = self.current?.payload();
            self.current = self.current?.next();
            Some(payload)
        }
    }
}

pub struct IterMut<'a, T> {
    current: *mut Node<T>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let payload = &mut (*self.current).payload;
                self.current = (*self.current).next;
                Some(payload)
            }
        }
    }
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.is_empty() {
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
    fn setup_list(n: usize) -> List<usize> {
        let mut list = List::new();
        for i in 0..n {
            list.push_back(i);
        }
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

    mod push {
        use super::*;

        #[test]
        fn test_push_back() {
            let mut list: List<u8> = List::new();
            assert!(list.is_empty(), "is_empty() returns `false` after creation");

            list.push_back(1);
            assert_eq!(list.len(), 1, "bad length after push_back()");
            assert_eq!(list.head(), Some(&1), "incorrect head after push_back()");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_back()");
            assert!(
                !list.is_empty(),
                "is_empty() returns `true` after push_back()"
            );

            list.push_back(2);
            assert_eq!(list.len(), 2, "bad length after push_back()");
            assert!(list.head().is_some(), "head is None after push_back()");
            assert_eq!(list.head(), Some(&1), "incorrect head payload");
            assert_eq!(list.last(), Some(&2), "incorrect last after push_back()");
            assert!(
                !list.is_empty(),
                "is_empty() returns `true` after push_back()"
            );

            let mut list: List<String> = List::new();
            list.push_back("hello".to_string());
            assert_eq!(list.len(), 1, "bad length after push_back()");
            assert!(list.head().is_some(), "head is None after push_back()");
            assert_eq!(list.head().unwrap(), "hello", "incorrect head payload");

            let mut list: List<&[char]> = List::new();
            list.push_back(&['a', 'b', 'c']);
            assert_eq!(list.len(), 1, "bad length after push_back()");
            assert!(list.head().is_some(), "head is None after push_back()");
            assert_eq!(
                list.head().unwrap(),
                &['a', 'b', 'c'],
                "incorrect head payload"
            );
        }

        #[test]
        fn test_push_to_empty_list_updates_head_and_last() {
            let mut list = List::new();

            list.push_back(100);
            assert_eq!(list.len(), 1);
            assert_eq!(list.head(), Some(&100));
            assert_eq!(list.last(), Some(&100));

            let mut list2 = List::new();
            list2.push_front(200);
            assert_eq!(list2.len(), 1);
            assert_eq!(list2.head(), Some(&200));
            assert_eq!(list2.last(), Some(&200));
        }

        #[test]
        fn test_push_front() {
            let mut list: List<u8> = List::new();
            assert!(list.is_empty(), "is_empty() returns `false` after creation");

            list.push_front(1);
            assert_eq!(list.len(), 1, "bad length after push_front()");
            assert_eq!(list.head(), Some(&1), "incorrect head after push_front()");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_front()");
            assert!(
                !list.is_empty(),
                "is_empty() returns `true` after push_front()"
            );

            list.push_front(2);
            assert_eq!(list.len(), 2, "bad length after push_front()");
            assert!(list.head().is_some(), "head is None after push_front()");
            assert_eq!(list.head(), Some(&2), "incorrect head payload");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_front()");
            assert!(
                !list.is_empty(),
                "is_empty() returns `true` after push_front()"
            );

            let mut list: List<String> = List::new();
            list.push_front("hello".to_string());
            assert_eq!(list.len(), 1, "bad length after push_front()");
            assert!(list.head().is_some(), "head is None after push_front()");
            assert_eq!(list.head().unwrap(), "hello", "incorrect head payload");

            let mut list: List<&[char]> = List::new();
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
            let mut list: List<u8> = List::new();
            assert!(list.is_empty(), "is_empty() returns `false` after creation");

            list.push_back(1);
            assert_eq!(list.len(), 1, "bad length after push_back()");
            assert_eq!(list.head(), Some(&1), "incorrect head after push_back()");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_back()");
            assert!(
                !list.is_empty(),
                "is_empty() returns `true` after push_back()"
            );

            list.push_front(2);
            assert_eq!(list.len(), 2, "bad length after push_front()");
            assert!(list.head().is_some(), "head is None after push_front()");
            assert_eq!(list.head(), Some(&2), "incorrect head payload");
            assert_eq!(list.last(), Some(&1), "incorrect last after push_front()");
            assert!(
                !list.is_empty(),
                "is_empty() returns `true` after push_front()"
            );

            list.push_back(3);
            assert_eq!(list.len(), 3, "bad length after push_back()");
            assert!(list.head().is_some(), "head is None after push_back()");
            assert_eq!(list.head(), Some(&2), "incorrect head payload");
            assert_eq!(list.last(), Some(&3), "incorrect last after push_back()");
            assert!(
                !list.is_empty(),
                "is_empty() returns `true` after push_back()"
            );
        }
    }

    mod pop {
        use super::*;

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
            assert_eq!(
                list.pop_back(),
                Some(42),
                "pop_back() should return the only element"
            );
            assert!(
                list.is_empty(),
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
            assert!(list.is_empty(), "list should be empty after all pop-backs");
        }

        #[test]
        fn test_pop_front_empty_list() {
            let mut list = List::<u8>::new();
            assert_eq!(
                list.pop_front(),
                None,
                "pop_front() from empty list should return None"
            );
            assert!(
                list.is_empty(),
                "list should remain empty after pop_front() on empty"
            );
        }

        #[test]
        fn test_pop_front_single_element() {
            let mut list = List::new();
            list.push_front(99);
            assert_eq!(
                list.pop_front(),
                Some(99),
                "pop_front() should return the only element"
            );
            assert!(
                list.is_empty(),
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
            assert!(list.is_empty(), "list should be empty after all pop_fronts");
        }
    }

    mod mixed {
        use super::*;

        #[test]
        fn test_mixed_push_pop_operations() {
            let mut list = List::new();

            list.push_back(1);
            list.push_front(0);
            list.push_back(2);

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
            assert_eq!(list.head(), Some(&1));
            assert_eq!(list.last(), Some(&1));

            // push_front(0)
            list.push_front(0);
            assert_eq!(list.head(), Some(&0));
            assert_eq!(list.last(), Some(&1));

            // push_back(2)
            list.push_back(2);
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
            let mut list = List::new();
            list.push_back("hello".to_string());
            list.push_back("world".to_string());

            assert_eq!(list.len(), 2);
            assert_eq!(list.head().unwrap(), "hello");
            assert_eq!(list.last().unwrap(), "world");

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
            assert_eq!(list.head().unwrap(), &vec![1, 2]);
            assert_eq!(list.last().unwrap(), &vec![3, 4]);

            let popped_front = list.pop_front().unwrap();
            assert_eq!(popped_front, vec![1, 2]);

            let popped_back = list.pop_back().unwrap();
            assert_eq!(popped_back, vec![3, 4]);
            assert!(list.is_empty());
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn test_empty_list_iterators() {
            let list: List<i32> = List::new();

            // Итератор по ссылкам
            {
                let mut iter = list.iter();
                assert_eq!(iter.next(), None);
            }

            // Изменяемый итератор
            {
                let mut iter_mut = list.iter_mut();
                assert_eq!(iter_mut.next(), None);
            }

            // IntoIterator (забирает владение)
            let into_iter = list.into_iter();
            assert_eq!(into_iter.collect::<Vec<_>>(), Vec::<i32>::new());
        }

        #[test]
        fn test_sequential_iteration() {
            let mut list = List::new();
            for i in 0..5 {
                list.push_back(i);
            }

            // Проверка итератора по ссылкам
            let collected: Vec<_> = list.iter().collect();
            assert_eq!(collected, vec![&0, &1, &2, &3, &4]);

            // Проверка изменяемого итератора (изменяем значения)
            for item in list.iter_mut() {
                *item *= 2;
            }
            let doubled: Vec<_> = list.iter().collect();
            assert_eq!(doubled, vec![&0, &2, &4, &6, &8]);

            // Проверка IntoIterator
            let into_collected: Vec<_> = list.into_iter().collect();
            assert_eq!(into_collected, vec![0, 2, 4, 6, 8]);
        }

        #[test]
        fn test_partial_iteration() {
            let mut list = List::new();
            for i in 0..10 {
                list.push_back(i);
            }

            {
                let mut iter = list.iter();
                // Проходим только первые 3 элемента
                assert_eq!(iter.next(), Some(&0));
                assert_eq!(iter.next(), Some(&1));
                assert_eq!(iter.next(), Some(&2));
                // Останавливаемся, не проходя до конца
            }

            // Список должен остаться целым
            assert_eq!(list.size, 10);

            // Можно начать итерацию заново
            let first_element = list.iter().next();
            assert_eq!(first_element, Some(&0));
        }

        #[test]
        fn test_concurrent_iterators() {
            let mut list = List::new();
            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            // Создаём несколько итераторов одновременно
            let iter1 = list.iter();
            let iter2 = list.iter();
            let mut iter3 = list.iter_mut();

            // Все итераторы должны работать независимо
            let collect1: Vec<_> = iter1.cloned().collect();
            let collect2: Vec<_> = iter2.cloned().collect();
            let first_mut = iter3.next().unwrap();
            *first_mut = 100;

            assert_eq!(collect1, vec![1, 2, 3]);
            assert_eq!(collect2, vec![1, 2, 3]);
            assert_eq!(list.iter().collect::<Vec<_>>(), vec![&100, &2, &3]);
        }

        #[test]
        fn test_mutable_iteration_modification() {
            let mut list = List::new();
            for i in 1..=3 {
                list.push_back(i);
            }

            let mut counter = 0;
            for item in list.iter_mut() {
                counter += 1;
                *item = *item * counter; // Умножаем на номер итерации
            }

            let result: Vec<_> = list.iter().copied().collect();
            assert_eq!(result, vec![1, 4, 9]); // 1×1, 2×2, 3×3
        }

        #[test]
        fn test_large_list_iteration() {
            const LARGE_SIZE: usize = 10_000;

            let mut list = List::new();
            for i in 0..LARGE_SIZE {
                list.push_back(i);
            }

            // Полный проход через итератор
            let sum: usize = list.iter().sum();
            let expected_sum: usize = (0..LARGE_SIZE).sum();
            assert_eq!(sum, expected_sum);

            // Частичная итерация с take()
            let first_10: Vec<_> = list.iter().take(10).copied().collect();
            assert_eq!(first_10, (0..10).collect::<Vec<_>>());
        }
    }

    mod insert {
        use super::*;

        #[test]
        fn test_insert_at_beginning_empty_list() {
            let mut list = List::new();
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
            let mut empty_list = List::new();
            assert!(
                empty_list.insert(1, 42).is_err(),
                "insert to empty list with index > 0 should return error"
            );
        }

        #[test]
        fn test_insert_with_complex_types_string() {
            let mut list = List::new();
            list.push_back("first".to_string());
            list.push_back("third".to_string());

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
            let mut list = List::new();

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
            let mut single_element = List::new();
            single_element.push_back(100);

            // Insert at beginning (should work)
            assert!(single_element.insert(0, 50).is_ok());
            assert_eq!(single_element.find(&50), Some(0));
            assert_eq!(single_element.find(&100), Some(1));

            // Insert at end (should work)
            assert!(single_element.insert(2, 150).is_ok());
            assert_eq!(single_element.find(&150), Some(2));
        }
    }

    mod remove {
        use super::*;

        #[test]
        fn test_remove_from_empty_list() {
            let mut list = List::<u8>::new();
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
            assert_eq!(list.find(&0), None, "0 should no longer be in the list");
        }

        #[test]
        fn test_remove_last_element() {
            let mut list = setup_list(3); // [0, 1, 2]
            let removed = list.remove(2).unwrap(); // index = size - 1
            assert_eq!(removed, 2, "removed value should be 2 (last element)");
            assert_eq!(list.len(), 2, "size should decrease by 1");
            assert_eq!(list.last(), Some(&1), "new last should be 1");
            assert_eq!(list.find(&2), None, "2 should no longer be in the list");
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
            let mut empty_list = List::<u8>::new();
            assert!(
                empty_list.remove(0).is_err(),
                "remove from empty list should return error"
            );
        }

        #[test]
        fn test_remove_single_element_list() {
            let mut list = List::new();
            list.push_back(42);
            let removed = list.remove(0).unwrap();
            assert_eq!(removed, 42, "removed value should be 42");
            assert!(
                list.is_empty(),
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
            let mut list = List::new();
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
            let mut two_elements = List::new();
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
            assert!(two_elements.is_empty());
        }
    }

    mod memory_leaks {
        use super::*;
        use drop_tracker::DropTracker;

        #[test]
        fn test_memory_leaks() {
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

        #[test]
        fn test_iterators_with_drop_tracker() {
            let mut tracker = DropTracker::new();

            let mut list = List::new();
            for i in 0..5 {
                list.push_back(tracker.track(i));
            }

            assert_eq!(tracker.alive().count(), 5);

            // Используем все типы итераторов последовательно
            {
                // Итератор по ссылкам
                let ref_count: usize = list.iter().count();
                assert_eq!(ref_count, 5);
            }

            {
                // Изменяемый итератор (изменяем все элементы)
                for item in list.iter_mut() {
                    **item += 10;
                }
            }

            {
                // IntoIterator — забираем владение
                let collected: Vec<_> = list.into_iter().collect();
                assert_eq!(collected, vec![10, 11, 12, 13, 14]);
            }

            // После IntoIterator список уничтожен
            assert_eq!(tracker.alive().count(), 0);
            assert_eq!(tracker.dropped().count(), 5);
        }

        #[test]
        fn test_memory_leaks_with_remove_operations() {
            let mut tracker = DropTracker::new();

            let mut list = List::new();
            for i in 0..50 {
                list.push_back(tracker.track(i));
            }

            assert_eq!(
                tracker.alive().count(),
                50,
                "50 elements should be alive after push_back"
            );

            // Удаляем элементы с разных позиций
            assert_eq!(list.remove(0).unwrap(), 0);
            assert_eq!(list.remove(48).unwrap(), 49); // последний элемент
            assert_eq!(list.remove(24).unwrap(), 25); // средний элемент

            assert_eq!(
                tracker.alive().count(),
                47,
                "After removing 3 elements, 47 should remain alive"
            );

            // Полностью очищаем список
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

            let mut list = List::new();
            list.push_back(tracker.track(1));
            list.push_back(tracker.track(3));

            assert_eq!(
                tracker.alive().count(),
                2,
                "2 elements should be alive initially"
            );

            // Вставляем элемент в середину
            list.insert(1, tracker.track(2)).unwrap();

            assert_eq!(
                tracker.alive().count(),
                3,
                "3 elements should be alive after insert"
            );

            // Вставляем в начало и конец
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

            let mut list = List::new();
            for i in 0..20 {
                list.push_back(tracker.track(i));
            }

            assert_eq!(tracker.alive().count(), 20, "20 elements should be alive");

            // Выполняем несколько операций удаления и добавления
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
                    // Просто отмечаем удаление
                }
            }

            let mut list = List::new();
            for i in 0..15 {
                list.push_back(tracker.track(ComplexStruct {
                    id: i,
                    data: format!("data_{}", i),
                }));
            }

            assert_eq!(
                tracker.alive().count(),
                15,
                "15 ComplexStruct elements should be alive"
            );

            // Удаляем несколько элементов
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

            let mut list = List::new();
            for i in 0..10 {
                list.push_back(tracker.track(i));
            }

            assert_eq!(tracker.alive().count(), 10, "10 elements should be alive");

            // Попытка удаления по неверному индексу (не должна вызывать утечек)
            assert!(list.remove(15).is_err());

            // Попытка вставки по неверному индексу
            assert!(list.insert(15, tracker.track(99)).is_err());

            assert_eq!(
                tracker.alive().count(),
                10,
                "10 elements should be alive (10 original + 1 attempted insert)"
            );

            // Очищаем список
            while list.len() > 0 {
                let _ = list.pop_front();
            }

            drop(list); // Явное удаление

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
    }
}
