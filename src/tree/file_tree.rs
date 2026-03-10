use std::path::{Component, Path};

use crate::{List, SortedList};
use super::node::Node;

pub struct FileTree<'a> {
    root: Node<'a>
}

impl<'a> FileTree<'a> {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn add(&mut self, path: &'a Path, is_file: bool) {
        if path.is_relative() {
            return; // TODO: Err
        }
        if is_file {
            if self.root.files.is_none() {
                self.root.files = Some(SortedList::new());
            }
        } else {
            if self.root.dirs.is_none() {
                self.root.dirs = Some(SortedList::new());
            }
        }
        let files = self.root.files.as_mut().unwrap();
        let dirs = self.root.dirs.as_mut().unwrap();
        for component in path.components() {
            if component == Component::RootDir {
                continue
            }
            if is_file {
                files.push(component);
            } else {
                dirs.push(component);
            }
        }
    }
}