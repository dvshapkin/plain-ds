use super::Node;

pub struct IterMut<'a, T> {
    current: *mut Node<T>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(head: *mut Node<T>) -> Self {
        Self {
            current: head,
            _marker: Default::default(),
        }
    }
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
