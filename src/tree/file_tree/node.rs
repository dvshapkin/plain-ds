use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use super::iter::Iter;

#[derive(Debug)]
pub struct Childs {
    pub dirs: Box<BTreeMap<String, DirNode>>,
    pub files: Box<BTreeSet<String>>,
}

impl Childs {
    pub fn new() -> Self {
        Self {
            dirs: Box::new(BTreeMap::new()),
            files: Box::new(BTreeSet::new())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dirs.is_empty() && self.files.is_empty()
    }
}

#[derive(Debug)]
pub struct DirNode {
    pub name: String,
    pub childs: Option<Box<Childs>>,
}

impl DirNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            childs: None,
        }
    }
    
    pub fn childs_count(&self) -> usize {
        if let Some(childs) = &self.childs {
            return childs.dirs.len() + childs.files.len();
        }
        0
    }

    /// Returns an iterator over the immutable items of the list.
    pub fn iter(&self) -> Option<impl Iterator<Item = PathBuf>> {
        if let Some(childs) = &self.childs {
            Some(Iter::new(childs))
        } else {
            None
        }
    }

    // /// Returns an iterator over the mutable items of the list.
    // pub fn iter_mut(&mut self) -> impl Iterator<Item = &'a mut T>;
    //
    // /// Returns an iterator that consumes the list.
    // pub fn into_iter(self) -> impl Iterator<Item = T>;
}

impl PartialEq for DirNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for DirNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        println!("DirNode = {}", size_of::<DirNode>());
        println!("Childs = {}", size_of::<Childs>());
    }
}
