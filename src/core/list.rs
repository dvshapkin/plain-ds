use super::error::Result;

/// This trait defines common API for all single_linked_list implementations.
pub trait List<'a, T: 'a> {
    /// Returns single_linked_list size.
    fn len(&self) -> usize;

    /// Checks if the single_linked_list is empty.
    fn is_empty(&self) -> bool;

    /// Returns the payload value of the first node in the single_linked_list.
    fn head(&self) -> Option<&T>;

    /// Returns the payload value of the last node in the single_linked_list.
    fn last(&self) -> Option<&T>;

    /// Returns a single_linked_list item by index, or error if index out of bounds.
    fn get(&self, index: usize) -> Result<&T>;

    /// Returns a mutable single_linked_list item by index, or error if index out of bounds.
    fn get_mut(&mut self, index: usize) -> Result<&mut T>;

    /// Returns an iterator over the immutable items of the single_linked_list.
    fn iter(&self) -> impl Iterator<Item = &'a T>;

    /// Returns an iterator over the mutable items of the single_linked_list.
    fn iter_mut(&mut self) -> impl Iterator<Item = &'a mut T>;

    /// Returns an iterator that consumes the single_linked_list.
    fn into_iter(self) -> impl Iterator<Item = T>;

    /// Adds a new node to the end of the single_linked_list.
    fn push_back(&mut self, payload: T);

    /// Adds a new node to the front of the single_linked_list.
    fn push_front(&mut self, payload: T);

    /// Removes a node from the end of the single_linked_list and returns its payload value.
    fn pop_back(&mut self) -> Option<T>;

    /// Removes a node from the front of the single_linked_list and returns its payload value.
    fn pop_front(&mut self) -> Option<T>;

    /// Insert a new node at the specified location in the single_linked_list.
    fn insert(&mut self, index: usize, payload: T) -> Result<()>;

    /// Removes a node from the specified location in the single_linked_list.
    fn remove(&mut self, index: usize) -> Result<T>;

    /// Removes all items from the single_linked_list.
    fn clear(&mut self);

    /// Finds the first node whose payload satisfies the predicate and returns its index.
    fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize>;

    /// Sorts the single_linked_list in ascending order.
    fn sort(&mut self)
    where
        T: PartialOrd + Default;
}
