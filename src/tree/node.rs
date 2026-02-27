use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ffi::OsString;

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    Dir,
    File,
}

pub struct Childs {
    pub dirs: Box<BTreeMap<OsString, Node>>,
    pub files: Box<BTreeMap<OsString, Node>>,
}

impl Childs {
    pub fn new() -> Self {
        Self {
            dirs: Box::new(BTreeMap::new()),
            files: Box::new(BTreeMap::new())
        }
    }
}

pub struct Node {
    pub name: OsString,
    pub r#type: ComponentType,
    pub childs: Option<Box<Childs>>,
}

impl Node {
    pub fn new(name: &str, r#type: ComponentType) -> Self {
        Self {
            name: OsString::from(name),
            r#type,
            childs: None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.r#type != other.r#type {
            return None;
        }
        self.name.partial_cmp(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        println!("Node = {}", size_of::<Node>());
        println!("Childs = {}", size_of::<Childs>());
    }
}
