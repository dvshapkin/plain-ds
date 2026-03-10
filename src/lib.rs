mod core;

#[cfg(feature = "list")]
pub mod list;

#[cfg(feature = "tree")]
pub mod tree;

pub use core::{DSError, Result};

#[cfg(feature = "list")]
pub use list::{List, SinglyLinkedList, SortedList};

#[cfg(feature = "tree")]
pub use tree::FileTree;
