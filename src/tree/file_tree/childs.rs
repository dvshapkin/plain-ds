use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use super::iter::Iter;

#[derive(Debug, Default)]
pub struct Childs {
    pub dirs: Box<BTreeMap<String, Childs>>,
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

    pub fn childs_count(&self) -> usize {
        self.dirs.len() + self.files.len()
    }

    /// Returns an iterator over the immutable items of the list.
    pub fn iter(&self) -> impl Iterator<Item = PathBuf> {
        Iter::new(self)
    }

    pub fn clear(&mut self) {
        self.dirs.clear();
        self.files.clear();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        println!("Childs = {}", size_of::<Childs>());
    }
}
