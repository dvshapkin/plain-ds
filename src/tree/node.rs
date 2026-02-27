use std::cmp::Ordering;
use std::path::Component;

use crate::SortedList;

pub struct Node<'a> {
    pub name: Component<'a>,
    pub files: Option<SortedList<Component<'a>>>,
    pub dirs: Option<SortedList<Component<'a>>>,
}

impl<'a> Node<'a> {
    pub fn new() -> Self {
        Self {
            name: Component::RootDir,
            files: None,
            dirs: None,
        }
    }
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
