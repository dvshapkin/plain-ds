use crate::core::node_one_link::node::Node;

pub struct Iter<'a, T> {
    current: *const Node<T>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(head: *const Node<T>) -> Self {
        Self {
            current: head,
            _marker: Default::default(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let payload = &(*self.current).payload;
                self.current = (*self.current).next;
                Some(payload)
            }
        }
    }
}