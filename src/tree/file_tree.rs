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
    pub fn add_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        if path.as_os_str() == "" {
            return Err(DSError::EmptyPath);
        }
        if !path.is_absolute() {
            return Err(DSError::NotAbsolutePath {
                path: path.to_string_lossy().into_owned(),
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

    /// Add file into tree.
    ///
    /// `path` must be absolute (i.e., start with `/`) and not contain prefixes
    /// (for example, like in Windows - `C:`).
    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        if path.as_os_str() == "" {
            return Err(DSError::EmptyPath);
        }
        if !path.is_absolute() {
            return Err(DSError::NotAbsolutePath {
                path: path.to_string_lossy().into_owned(),
            });
        }
        if path.as_os_str() == "/" {
            return Ok(());
        }

        let mut components: Vec<_> = path.components().collect();
        let file_component = components.pop().ok_or(DSError::EmptyPath)?;

        // First pass: create all necessary directories
        let last_node = self.ensure_dirs(&components);

        // Second pass: add the file to the last directory
        let name = file_component.as_os_str().to_string_lossy().to_string();
        let childs = last_node
            .childs
            .get_or_insert_with(|| Box::new(Childs::new()));

        let node = Node::new(&name, ComponentType::File);
        childs.files.insert(name, node);

        Ok(())
    }

    fn ensure_dirs(&mut self, components: &[Component<'_>]) -> &mut Node {
        let mut current = &mut self.root;

        // Skip RootDir
        for component in components.iter().skip(1) {
            let name = component.as_os_str().to_string_lossy().to_string();

            let childs = current
                .childs
                .get_or_insert_with(|| Box::new(Childs::new()));
            current = childs
                .dirs
                .entry(name.clone())
                .or_insert_with(|| Node::new(&name, ComponentType::Dir));
        }

        current
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod add_dir_file {
        use super::*;

        /// Test adding a simple directory at the root level.
        #[test]
        fn test_add_simple_dir() {
            let mut tree = FileTree::new();

            // Add /home directory
            assert!(tree.add_dir("/home").is_ok());

            // Verify that /home directory exists
            let root_childs = tree.root.childs.as_ref().unwrap();
            assert!(root_childs.dirs.contains_key("home"));
        }

        /// Test adding nested directories.
        #[test]
        fn test_add_nested_dirs() {
            let mut tree = FileTree::new();

            // Add nested directories: /home/user/documents
            assert!(tree.add_dir(Path::new("/home/user/documents")).is_ok());

            // Verify the full path exists
            let home = tree.root.childs.as_ref().unwrap().dirs.get("home").unwrap();
            let user = home.childs.as_ref().unwrap().dirs.get("user").unwrap();
            assert!(user.childs.as_ref().unwrap().dirs.contains_key("documents"));
        }

        /// Test adding directory with root path (should succeed without changes).
        #[test]
        fn test_add_root_dir() {
            let mut tree = FileTree::new();

            // Adding root directory should be a no‑op
            assert!(tree.add_dir(Path::new("/")).is_ok());

            // Root should still exist and have no children
            assert_eq!(tree.root.name, "/");
            assert!(tree.root.childs.is_none());
        }

        /// Test adding file to an existing directory structure.
        #[test]
        fn test_add_file_to_existing_dirs() {
            let mut tree = FileTree::new();

            // First create directories
            assert!(tree.add_dir(Path::new("/home/user")).is_ok());
            // Then add a file
            assert!(tree.add_file(Path::new("/home/user/document.txt")).is_ok());

            // Verify file exists in the correct location
            let home = tree.root.childs.as_ref().unwrap().dirs.get("home").unwrap();
            let user = home.childs.as_ref().unwrap().dirs.get("user").unwrap();
            assert!(user.childs.as_ref().unwrap().files.contains_key("document.txt"));
        }

        /// Test adding file creates necessary intermediate directories.
        #[test]
        fn test_add_file_creates_intermediate_dirs() {
            let mut tree = FileTree::new();

            // Add file — this should create /projects/rust/ directories automatically
            assert!(tree.add_file(Path::new("/projects/rust/main.rs")).is_ok());

            // Verify full path was created
            let projects = tree.root.childs.as_ref().unwrap().dirs.get("projects").unwrap();
            let rust = projects.childs.as_ref().unwrap().dirs.get("rust").unwrap();
            assert!(rust.childs.as_ref().unwrap().files.contains_key("main.rs"));
        }

        /// Test error when adding non‑absolute path for directory.
        #[test]
        fn test_add_non_absolute_dir_path_error() {
            let mut tree = FileTree::new();

            // Relative path should return error
            let result = tree.add_dir(Path::new("relative/path"));
            assert!(result.is_err());
            if let Err(DSError::NotAbsolutePath { path }) = result {
                assert_eq!(path, "relative/path");
            } else {
                panic!("Expected NotAbsolutePath error");
            }
        }

        /// Test error when adding non‑absolute path for file.
        #[test]
        fn test_add_non_absolute_file_path_error() {
            let mut tree = FileTree::new();

            // Relative path should return error
            let result = tree.add_file(Path::new("document.txt"));
            assert!(result.is_err());
            if let Err(DSError::NotAbsolutePath { path }) = result {
                assert_eq!(path, "document.txt");
            } else {
                panic!("Expected NotAbsolutePath error");
            }
        }

        /// Test adding multiple files in the same directory.
        #[test]
        fn test_add_multiple_files_same_dir() {
            let mut tree = FileTree::new();

            // Add multiple files to /tmp directory
            assert!(tree.add_file(Path::new("/tmp/file1.txt")).is_ok());
            assert!(tree.add_file(Path::new("/tmp/file2.txt")).is_ok());
            assert!(tree.add_file(Path::new("/tmp/script.sh")).is_ok());

            // Verify all files exist
            let tmp = tree.root.childs.as_ref().unwrap().dirs.get("tmp").unwrap();
            let files = &tmp.childs.as_ref().unwrap().files;
            assert!(files.contains_key("file1.txt"));
            assert!(files.contains_key("file2.txt"));
            assert!(files.contains_key("script.sh"));
        }

        /// Test idempotent behavior — adding the same path multiple times.
        #[test]
        fn test_idempotent_add_operations() {
            let mut tree = FileTree::new();

            // Add the same directory twice
            assert!(tree.add_dir(Path::new("/var/log")).is_ok());
            assert!(tree.add_dir(Path::new("/var/log")).is_ok()); // Should not fail

            // Add the same file twice
            assert!(tree.add_file(Path::new("/var/log/system.log")).is_ok());
            // Second add should overwrite or be idempotent
            assert!(tree.add_file(Path::new("/var/log/system.log")).is_ok());

            // Verify structure is correct
            let var = tree.root.childs.as_ref().unwrap().dirs.get("var").unwrap();
            let log = var.childs.as_ref().unwrap().dirs.get("log").unwrap();
            assert!(log.childs.as_ref().unwrap().files.contains_key("system.log"));
        }

        /// Test handling of paths with special characters.
        #[test]
        fn test_add_path_with_special_chars() {
            let mut tree = FileTree::new();

            // Add directory with special characters
            assert!(tree.add_dir(Path::new("/special-@#$%/test")).is_ok());

            // Verify it was added correctly
            let special = tree.root.childs.as_ref().unwrap().dirs.get("special-@#$%").unwrap();
            assert!(special.childs.as_ref().unwrap().dirs.contains_key("test"));
        }

        /// Test empty path handling.
        #[test]
        fn test_empty_path_handling() {
            let mut tree = FileTree::new();

            // Empty path should be handled gracefully
            let empty_path = Path::new("");
            let result = tree.add_file(empty_path);
            assert!(result.is_err());
            if let Err(DSError::EmptyPath) = result {
                // Expected error type
            } else {
                panic!("Expected EmptyPath error for empty path");
            }
        }
    }
}
