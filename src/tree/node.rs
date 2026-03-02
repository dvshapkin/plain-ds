use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

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
