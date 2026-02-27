use crate::core::{DSError, Result};

/// This trait defines common API for all list implementations.
pub trait List<'a, T: 'a> {
    /// Returns list size.
    fn len(&self) -> usize;

    /// Checks if the list is empty.
    ///
    /// Efficiency: O(1)
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the payload value of the first node in the list.
    fn head(&self) -> Option<&T>;

    /// Returns the payload value of the last node in the list.
    fn last(&self) -> Option<&T>;

    /// Returns a list item by index, or error if index out of bounds.
    ///
    /// Efficiency: O(n)
    fn get(&self, index: usize) -> Result<&'a T> {
        self.iter().nth(index).ok_or(DSError::IndexOutOfBounds {
            index,
            len: self.len(),
        })
    }

    /// Returns a mutable list item by index, or error if index out of bounds.
    ///
    /// Efficiency: O(n)
    fn get_mut(&mut self, index: usize) -> Result<&'a mut T> {
        let list_size = self.len();
        self.iter_mut().nth(index).ok_or(DSError::IndexOutOfBounds {
            index,
            len: list_size
        })
    }

    /// Returns an iterator over the immutable items of the list.
    fn iter(&self) -> impl Iterator<Item = &'a T>;

    /// Returns an iterator over the mutable items of the list.
    fn iter_mut(&mut self) -> impl Iterator<Item = &'a mut T>;

    /// Returns an iterator that consumes the list.
    fn into_iter(self) -> impl Iterator<Item = T>;

    /// Adds a new node to the list.
    fn push(&mut self, payload: T);

    /// Removes a node from the end of the list and returns its payload value.
    fn pop_back(&mut self) -> Option<T>;

    /// Removes a node from the front of the list and returns its payload value.
    fn pop_front(&mut self) -> Option<T>;

    /// Removes a node from the specified location in the list.
    fn remove(&mut self, index: usize) -> Result<T>;

    /// Removes all items from the list.
    ///
    /// Efficiency: O(n)
    fn clear(&mut self) {
        while self.len() != 0 {
            let _ = self.pop_front();
        }
    }

    /// Finds the first node whose payload is equal to the given `value` and returns its index.
    /// Returns `None` if there is no such node.
    ///
    /// Efficiency: O(n)
    fn find(&self, value: &T) -> Option<usize>
    where T: PartialEq<T>
    {
        self.iter()
            .enumerate()
            .find(|(_, item)| *item == value)
            .map(|(index, _)| index)
    }
}
