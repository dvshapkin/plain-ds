use super::error::Result;

pub trait List<'a, T: 'a> {
    /// Returns list size.
    fn len(&self) -> usize;

    /// Checks if the list is empty.
    fn is_empty(&self) -> bool;

    /// Returns the payload value of the first node in the list.
    fn head(&self) -> Option<&T>;

    /// Returns the payload value of the last node in the list.
    fn last(&self) -> Option<&T>;

    /// Returns a list item by index, or error if index out of bounds.
    fn get(&self, index: usize) -> Result<&T>;

    /// Returns a mutable list item by index, or error if index out of bounds.
    fn get_mut(&mut self, index: usize) -> Result<&mut T>;

    /// Returns an iterator over the immutable items of the list.
    fn iter(&self) -> impl Iterator<Item = &'a T>;

    /// Returns an iterator over the mutable items of the list.
    fn iter_mut(&mut self) -> impl Iterator<Item = &'a mut T>;

    /// Returns an iterator that consumes the list.
    fn into_iter(self) -> impl Iterator<Item = T>;

    /// Adds a new node to the end of the list.
    fn push_back(&mut self, payload: T);

    /// Adds a new node to the front of the list.
    fn push_front(&mut self, payload: T);

    /// Removes a node from the end of the list and returns its payload value.
    fn pop_back(&mut self) -> Option<T>;

    /// Removes a node from the front of the list and returns its payload value.
    fn pop_front(&mut self) -> Option<T>;

    /// Insert a new node at the specified location in the list.
    fn insert(&mut self, index: usize, payload: T) -> Result<()>;

    /// Removes a node from the specified location in the list.
    fn remove(&mut self, index: usize) -> Result<T>;

    /// Removes all items from the list.
    fn clear(&mut self);

    /// Finds the first node whose payload satisfies the predicate and returns its index.
    fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize>;

    /// Sorts the list in ascending order.
    fn sort(&mut self);
}
