use std::collections::{btree_map, btree_set};
use std::path::{Path, PathBuf};

use super::childs::Childs;

pub struct Iter<'a> {
    parent: &'a Path,
    iter_dirs: btree_map::Iter<'a, String, Childs>,
    iter_files: btree_set::Iter<'a, String>,
}

impl<'a> Iter<'a> {
    pub fn new(childs: &'a Childs) -> Self {
        Self {
            parent: Path::new("/"),
            iter_dirs: childs.dirs.iter(),
            iter_files: childs.files.iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((path, _)) = self.iter_dirs.next() {
            return Some(self.parent.join(Path::new(path)));
        } else {
            if let Some(path) = self.iter_files.next() {
                return Some(self.parent.join(Path::new(path)));
            } else {
                None
            }
        }
    }
}
