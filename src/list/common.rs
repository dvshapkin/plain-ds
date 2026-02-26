use crate::core::List;
use crate::core::node_one_link::Node;

#[inline]
#[allow(unused_assignments, unused)]
pub fn push_back<'a, T: 'a>(
    list: &impl List<'a, T>,
    head: &mut *mut Node<T>,
    last: &mut *mut Node<T>,
    payload: T,
) {
    let ptr = Box::into_raw(Box::new(Node::new(payload)));
    if list.is_empty() {
        *head = ptr;
    } else {
        unsafe { (**last).next = ptr };
    }
    *last = ptr;
}
