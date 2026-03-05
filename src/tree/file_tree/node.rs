use std::collections::{btree_map, btree_set, BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::ptr;

#[derive(Debug, Default)]
pub struct DirNode {
    dirs: *mut BTreeMap<String, DirNode>,
    files: *mut BTreeSet<String>,
}

impl DirNode {
    pub fn new() -> Self {
        Self {
            dirs: ptr::null_mut(),
            files: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.childs_count() == 0
    }

    pub fn childs_count(&self) -> usize {
        let dirs_count = if self.dirs.is_null() {
            0
        } else {
            unsafe { (*self.dirs).len() }
        };
        let files_count = if self.files.is_null() {
            0
        } else {
            unsafe { (*self.files).len() }
        };
        dirs_count + files_count
    }

    pub fn dirs_contains(&self, name: &str) -> bool {
        if self.dirs.is_null() {
            false
        } else {
            unsafe { (*self.dirs).contains_key(name) }
        }
    }

    pub fn files_contains(&self, name: &str) -> bool {
        if self.files.is_null() {
            false
        } else {
            unsafe { (*self.files).contains(name) }
        }
    }

    pub fn get_dir(&self, name: &str) -> Option<&DirNode> {
        if self.dirs.is_null() {
            None
        } else {
            unsafe { (*self.dirs).get(name) }
        }
    }

    pub fn get_dir_mut(&self, name: &str) -> Option<&mut DirNode> {
        if self.dirs.is_null() {
            None
        } else {
            unsafe { (*self.dirs).get_mut(name) }
        }
    }

    pub fn insert_file<T: Into<String>>(&mut self, name: T) {
        if self.files.is_null() {
            self.files = Box::into_raw(Box::new(BTreeSet::new()));
        }
        unsafe { (*self.files).insert(name.into()) };
    }

    pub fn insert_dir<T: Into<String>>(&mut self, name: T) {
        if self.dirs.is_null() {
            self.dirs = Box::into_raw(Box::new(BTreeMap::new()));
        }
        let name = name.into();
        unsafe {
            if !(*self.dirs).contains_key(&name) {
                (*self.dirs).insert(name, DirNode::new());
            }
        }
    }

    pub fn remove_file(&mut self, name: &str) {
        if !self.files.is_null() {
            unsafe {
                (*self.files).remove(name);
            }
        }
    }

    pub fn remove_dir(&mut self, name: &str) {
        if !self.dirs.is_null() {
            unsafe {
                (*self.dirs).remove(name);
            }
        }
    }

    /// Returns an iterator over the immutable items of the list.
    pub fn iter(&self) -> impl Iterator<Item = PathBuf> {
        Iter::new(self)
    }

    pub fn files_iter(&self) -> impl Iterator<Item = &String> {
        let iter_opt = if self.files.is_null() {
            None
        } else {
            Some(unsafe { (*self.files).iter() })
        };
        iter_opt.into_iter().flatten()
    }

    pub fn dirs_iter(&self) -> impl Iterator<Item = (&String, &DirNode)> {
        let iter_opt = if self.dirs.is_null() {
            None
        } else {
            Some(unsafe { (*self.dirs).iter() })
        };
        iter_opt.into_iter().flatten()
    }

    pub fn clear(&mut self) {
        if !self.dirs.is_null() {
            unsafe { (*self.dirs).clear() };
        }
        if !self.files.is_null() {
            unsafe { (*self.files).clear() };
        }
    }
}

pub struct Iter<'a> {
    parent: &'a Path,
    iter_dirs: btree_map::Iter<'a, String, DirNode>,
    iter_files: btree_set::Iter<'a, String>,
}

impl<'a> Iter<'a> {
    pub fn new(node: &'a DirNode) -> Self {
        Self {
            parent: Path::new("/"),
            iter_dirs: unsafe { (*node.dirs).iter() },
            iter_files: unsafe { (*node.files).iter() },
        }
    }
}

impl Drop for DirNode {
    fn drop(&mut self) {
        if !self.dirs.is_null() {
            let _ = unsafe { Box::from_raw(self.dirs) };
        }
        if !self.files.is_null() {
            let _ = unsafe { Box::from_raw(self.files) };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        println!("DirNode = {}", size_of::<DirNode>());
    }
}
