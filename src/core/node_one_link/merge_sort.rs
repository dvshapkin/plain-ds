use crate::core::node_one_link::node::Node;

/// Merge sort implementation for linked single_linked nodes
pub fn merge_sort<T>(head: *mut Node<T>) -> *mut Node<T>
where
    T: PartialOrd + Default
{
    // Base case: empty single_linked or single node
    if head.is_null() || unsafe { (*head).next.is_null() } {
        return head;
    }

    // Split the single_linked into two halves
    let (left, right) = split_list(head);

    // Recursively sort both halves
    let left_sorted = merge_sort(left);
    let right_sorted = merge_sort(right);

    // Merge the sorted halves
    merge(left_sorted, right_sorted)
}

/// Splits the single_linked into two approximately equal halves
fn split_list<T>(head: *mut Node<T>) -> (*mut Node<T>, *mut Node<T>) {
    let mut slow = head;
    let mut fast = unsafe { (*head).next };

    // Use fast and slow pointers to find the middle
    while !fast.is_null() {
        fast = unsafe { (*fast).next };
        if !fast.is_null() {
            slow = unsafe { (*slow).next };
            fast = unsafe { (*fast).next };
        }
    }

    // Split at the slow pointer
    let right_head = unsafe { (*slow).next };
    unsafe { (*slow).next = std::ptr::null_mut() };

    (head, right_head)
}

/// Merges two sorted linked lists into one sorted single_linked
fn merge<T>(mut left: *mut Node<T>, mut right: *mut Node<T>) -> *mut Node<T>
where
    T: PartialOrd + Default
{
    // Dummy node to simplify merging logic
    let dummy = Box::new(Node {
        payload: T::default(), // Placeholder, will be ignored
        next: std::ptr::null_mut(),
    });
    let tail = Box::into_raw(dummy);

    // Keep track of the actual head (skip dummy)
    let mut actual_tail = tail;

    while !left.is_null() && !right.is_null() {
        unsafe {
            if (*left).payload <= (*right).payload {
                // Take from left single_linked
                (*actual_tail).next = left;
                actual_tail = left;
                left = (*left).next;
            } else {
                // Take from right single_linked
                (*actual_tail).next = right;
                actual_tail = right;
                right = (*right).next;
            }
        }
    }

    // Attach remaining nodes
    if !left.is_null() {
        unsafe { (*actual_tail).next = left };
    } else if !right.is_null() {
        unsafe { (*actual_tail).next = right };
    }

    // The real head is the next of dummy node
    let result_head = unsafe { (*tail).next };

    // Free the dummy node
    let _ = unsafe { Box::from_raw(tail) };

    result_head
}