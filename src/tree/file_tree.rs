//! This module contains file-tree implementation.

use super::node::{Childs, ComponentType, Node};
use crate::{DSError, Result};
use std::path::{Component, Path};

/// `FileTree` is a data structure for compactly storing in memory hierarchical objects
/// such as files and directories. It also provides fast search and access to data.
///
/// **Implementation Features** <br>
/// If all the file paths you plan to store in `FileTree` begin with the same long prefix,
/// it's better to store this prefix separately, outside of this structure.
///
/// For example, you have several file paths:
///```plain text
/// /very/long/prefix/to/my/files/file.01
///
/// /very/long/prefix/to/my/files/a/file.02
///
/// /very/long/prefix/to/my/files/b/c/file.03
///```
///
/// Common prefix is: `/very/long/prefix/to/my/files` - store it separately.
///
/// And in `FileTree` store short paths: `/file.01`, `/a/file.02` and `/b/c/file.03`.
///
/// In this case, `FileTree` will store the following hierarchy:
///```plain text
///                    /
///        +-----------+------------+
///     file.01        a            b
///                    +            +
///                 file.02         c
///                                 +
///                              file.03
///```
/// All paths in `FileTree` must be absolute (i.e., start with `/`). <br>
/// Do not include any prefixes into paths (for example, like in Windows - `C:`).
pub struct FileTree {
    root: Node,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            root: Node::new("/", ComponentType::Dir),
        }
    }

    /// Add directory into tree.
    ///
    /// `path` must be absolute (i.e., start with `/`) and not contain prefixes
    /// (for example, like in Windows - `C:`).
    pub fn add_dir(&mut self, path: &Path) -> Result<()> {
        if !path.is_absolute() {
            return Err(DSError::NotAbsolutePath {
                path: path.as_os_str().to_owned(),
            });
        }
        if path.as_os_str() == "/" {
            return Ok(());
        }

        let components: Vec<_> = path.components().collect();

        // Create all necessary directories
        let _ = self.ensure_dirs(&components);

        Ok(())
    }

    pub fn add_file(&mut self, path: &Path) -> Result<()> {
        if !path.is_absolute() {
            return Err(DSError::NotAbsolutePath {
                path: path.as_os_str().to_owned(),
            });
        }
        if path.as_os_str() == "/" {
            return Ok(());
        }

        let components: Vec<_> = path.components().collect();

        // First pass: create all necessary directories
        let last_node = self.ensure_dirs(&components);

        // Second pass: add the file to the last directory
        let last_component = components.last().ok_or(DSError::EmptyPath)?;
        let name = last_component.as_os_str();
        let childs = last_node
            .childs
            .get_or_insert_with(|| Box::new(Childs::new()));

        let node = Node::new(name.to_string_lossy().as_ref(), ComponentType::File);
        childs.files.insert(name.to_owned(), node);

        Ok(())
    }

    fn ensure_dirs(&mut self, components: &[Component<'_>]) -> &mut Node {
        let mut current = &mut self.root;

        // Skip RootDir and the last component (file)
        for component in components.iter().skip(1).take(components.len() - 2) {
            let name = component.as_os_str();

            let childs = current
                .childs
                .get_or_insert_with(|| Box::new(Childs::new()));
            current = childs
                .dirs
                .entry(name.to_owned())
                .or_insert_with(|| Node::new(name.to_string_lossy().as_ref(), ComponentType::Dir));
        }

        current
    }
}
