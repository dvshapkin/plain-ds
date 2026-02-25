mod core;
pub mod fast;
pub mod light;

pub use core::error::{DSError, Result};
pub use light::SingleLinkedList;
pub use fast::OrderedList;
