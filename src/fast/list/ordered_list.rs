use crate::core::node_one_link::node::Node;

pub struct OrderedList<T> {
    head: *mut Node<T>, // 8 bytes
    last: *mut Node<T>, // 8 bytes
    size: usize,        // 8 bytes
}