//! This module contains file-tree implementation.

use super::node::{Childs, DirNode};
use crate::{DSError, Result};
use std::path::{Component, Path};

/// `FileTree` is a specialized data structure for compactly storing in memory hierarchical
/// structure of files and directories. It also provides fast search and access to data.
///
/// **Implementation Features** <br>
/// If all the file paths you plan to store in `FileTree` begin with the same long prefix,
/// it's better to store this prefix separately, outside of this structure.
///
/// For example, you have several file paths:
///```plain text
/// /very/long/prefix/to/my/files/file.01
///
/// /very/long/prefix/to/my/files/alfa/file.02
///
/// /very/long/prefix/to/my/files/beta/gamma/file.03
///```
///
/// Common prefix is: `/very/long/prefix/to/my/files` - store it separately.
///
/// And in `FileTree` store short paths: `/file.01`, `/alfa/file.02` and `/beta/gamma/file.03`.
///
/// In this case, `FileTree` will store the following hierarchy:
///```plain text
///                      /
///        +-------------+--------------+
///     file.01         alfa           beta
///                      /              /
///                   file.02         gamma
///                                     /
///                                  file.03
///```
/// All paths in `FileTree` must be absolute (i.e., start with `/`). <br>
/// Do not include any prefixes into paths (for example, like in Windows - `C:`).
pub struct FileTree {
    root: DirNode,
}

impl FileTree {
    /// Creates new file-tree and initialize root as `/`.
    pub fn new() -> Self {
        Self {
            root: DirNode::new("/"),
        }
    }

    /// Checks if `path` is contained in the tree as file.
    ///
    /// **Efficiency**: O(n), where `n` is a path length (in components).
    pub fn contains_file<P: AsRef<Path>>(&self, path: P) -> Result<bool> {
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
            return Ok(true);
        }

        // Skip RootDir
        let mut components: Vec<_> = path.components().skip(1).collect();
        let file_component = components.pop().ok_or(DSError::EmptyPath)?;

        Ok(self.check_path(&components, Some(file_component)))
    }

    /// Checks if `path` is contained in the tree as directory.
    ///
    /// **Efficiency**: O(n), where `n` is a path length (in components).
    pub fn contains_dir<P: AsRef<Path>>(&self, path: P) -> Result<bool> {
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
            return Ok(true);
        }

        // Skip RootDir
        let components: Vec<_> = path.components().skip(1).collect();

        Ok(self.check_path(&components, None))
    }

    /// Add directory into tree.
    ///
    /// `path` must be absolute (i.e., start with `/`) and not contain prefixes
    /// (for example, like in Windows - `C:`).
    ///
    /// **Efficiency**: O(n), where `n` is a path length (in components).
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

        // Skip RootDir
        let components: Vec<_> = path.components().skip(1).collect();

        // Create all necessary directories
        let _ = self.ensure_dirs(&components);

        Ok(())
    }

    /// Add file into tree.
    ///
    /// `path` must be absolute (i.e., start with `/`) and not contain prefixes
    /// (for example, like in Windows - `C:`).
    ///
    /// **Efficiency**: O(n), where `n` is a path length (in components).
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

        // Skip RootDir
        let mut components: Vec<_> = path.components().skip(1).collect();
        let file_component = components.pop().ok_or(DSError::EmptyPath)?;

        // First pass: create all necessary directories
        let last_node = self.ensure_dirs(&components);

        // Second pass: add the file to the last directory
        let name = file_component.as_os_str().to_string_lossy().to_string();
        let childs = last_node
            .childs
            .get_or_insert_with(|| Box::new(Childs::new()));

        childs.files.insert(name);

        Ok(())
    }

    /// Clears all tree contents.
    ///
    /// **Efficiency**: O(1)
    pub fn clear(&mut self) {
        if let Some(_) = self.root.childs.take() {}
    }

    fn check_path(&self, components: &[Component<'_>], file_component: Option<Component<'_>>) -> bool {
        let mut current = &self.root;

        // First pass: checks all parent directories
        let mut is_found = true;
        for component in components {
            let name = component.as_os_str().to_string_lossy().to_string();

            if let Some(childs) = &current.childs {
                if !childs.dirs.contains_key(&name) {
                    is_found = false;
                    break;
                }
                current = &childs.dirs[&name];
            } else {
                is_found = false;
                break;
            }
        }

        // Second pass: checks the file in the last directory
        if let Some(file_component) = file_component && is_found {
            let name = file_component.as_os_str().to_string_lossy().to_string();
            if let Some(childs) = &current.childs {
                if !childs.files.contains(&name) {
                    is_found = false;
                }
            } else {
                is_found = false;
            }
        }

        is_found
    }

    fn ensure_dirs(&mut self, components: &[Component<'_>]) -> &mut DirNode {
        let mut current = &mut self.root;

        for component in components {
            let name = component.as_os_str().to_string_lossy().to_string();

            let childs = current
                .childs
                .get_or_insert_with(|| Box::new(Childs::new()));
            current = childs
                .dirs
                .entry(name.clone())
                .or_insert_with(|| DirNode::new(&name));
        }

        current
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod add {
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
            assert!(user.childs.as_ref().unwrap().files.contains("document.txt"));
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
            assert!(rust.childs.as_ref().unwrap().files.contains("main.rs"));
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
            assert!(files.contains("file1.txt"));
            assert!(files.contains("file2.txt"));
            assert!(files.contains("script.sh"));
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
            assert!(log.childs.as_ref().unwrap().files.contains("system.log"));
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

    mod contains {
        use super::*;

        /// Test that root path is always contained in the tree.
        #[test]
        fn test_contains_root_path() {
            let tree = FileTree::new();

            assert_eq!(tree.contains_dir("/"), Ok(true));
            assert_eq!(tree.contains_file("/"), Ok(true)); // Root can be considered as existing
        }

        /// Test checking existence of a simple directory.
        #[test]
        fn test_contains_simple_dir() {
            let mut tree = FileTree::new();
            tree.add_dir(Path::new("/home")).unwrap();

            assert_eq!(tree.contains_dir("/home"), Ok(true));
            assert_eq!(tree.contains_file("/home"), Ok(false)); // Not a file
        }

        /// Test checking existence of a nested directory.
        #[test]
        fn test_contains_nested_dir() {
            let mut tree = FileTree::new();
            tree.add_dir(Path::new("/home/user/documents")).unwrap();

            assert_eq!(tree.contains_dir("/home"), Ok(true));
            assert_eq!(tree.contains_dir("/home/user"), Ok(true));
            assert_eq!(tree.contains_dir("/home/user/documents"), Ok(true));
        }

        /// Test checking existence of a file.
        #[test]
        fn test_contains_file() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/home/user/document.txt")).unwrap();

            assert_eq!(tree.contains_file("/home/user/document.txt"), Ok(true));
            assert_eq!(tree.contains_dir("/home/user/document.txt"), Ok(false)); // It's a file, not a directory
        }

        /// Test checking non‑existent path.
        #[test]
        fn test_contains_nonexistent_path() {
            let tree = FileTree::new();

            assert_eq!(tree.contains_dir("/nonexistent"), Ok(false));
            assert_eq!(tree.contains_file("/home/user/file.txt"), Ok(false));
        }

        /// Test checking file existence in non‑existent directory.
        #[test]
        fn test_contains_file_in_nonexistent_dir() {
            let mut tree = FileTree::new();
            // Add only the parent directory
            tree.add_dir(Path::new("/home")).unwrap();

            // File doesn't exist
            assert_eq!(tree.contains_file("/home/user/document.txt"), Ok(false));

            // Directory doesn't exist
            assert_eq!(tree.contains_dir("/home/user"), Ok(false));
        }

        /// Test error when checking empty path.
        #[test]
        fn test_contains_empty_path_error() {
            let tree = FileTree::new();

            let result = tree.contains_dir("");
            assert!(result.is_err());
            if let Err(DSError::EmptyPath) = result {
                // Expected error type
            } else {
                panic!("Expected EmptyPath error for empty path");
            }
        }

        /// Test error when checking non‑absolute path.
        #[test]
        fn test_contains_non_absolute_path_error() {
            let tree = FileTree::new();

            let result = tree.contains_dir("relative/path");
            assert!(result.is_err());
            if let Err(DSError::NotAbsolutePath { path }) = result {
                assert_eq!(path, "relative/path");
            } else {
                panic!("Expected NotAbsolutePath error");
            }
        }

        /// Test checking multiple paths in a complex tree structure.
        #[test]
        fn test_contains_multiple_paths_complex_tree() {
            let mut tree = FileTree::new();

            // Build a complex tree
            tree.add_dir(Path::new("/etc")).unwrap();
            tree.add_dir(Path::new("/var/log")).unwrap();
            tree.add_file(Path::new("/etc/config")).unwrap();
            tree.add_file(Path::new("/var/log/system.log")).unwrap();

            // Test various paths
            assert_eq!(tree.contains_dir("/etc"), Ok(true));
            assert_eq!(tree.contains_dir("/var"), Ok(true));
            assert_eq!(tree.contains_dir("/var/log"), Ok(true));
            assert_eq!(tree.contains_file("/etc/config"), Ok(true));
            assert_eq!(tree.contains_file("/var/log/system.log"), Ok(true));
            assert_eq!(tree.contains_file("/etc/passwd"), Ok(false));
            assert_eq!(tree.contains_dir("/tmp"), Ok(false));
        }

        /// Test checking path with special characters.
        #[test]
        fn test_contains_path_with_special_chars() {
            let mut tree = FileTree::new();

            tree.add_dir(Path::new("/special-@#$%")).unwrap();
            tree.add_file(Path::new("/special-@#$%/test.file")).unwrap();

            assert_eq!(tree.contains_dir("/special-@#$%"), Ok(true));
            assert_eq!(tree.contains_file("/special-@#$%/test.file"), Ok(true));
            assert_eq!(tree.contains_file("/special-@#$%/nonexistent"), Ok(false));
        }

        /// Test checking directory that exists as a file (should return false).
        #[test]
        fn test_contains_dir_but_is_file() {
            let mut tree = FileTree::new();

            // Add a file that would conflict with directory name
            tree.add_file(Path::new("/conflicted")).unwrap();

            // Should not be found as a directory
            assert_eq!(tree.contains_dir("/conflicted"), Ok(false));
            // But should be found as a file
            assert_eq!(tree.contains_file("/conflicted"), Ok(true));
        }

        /// Test checking file that exists as a directory (should return false).
        #[test]
        fn test_contains_file_but_is_dir() {
            let mut tree = FileTree::new();

            // Add a directory that would conflict with file name
            tree.add_dir(Path::new("/conflicted")).unwrap();

            // Should not be found as a file
            assert_eq!(tree.contains_file("/conflicted"), Ok(false));
            // But should be found as a directory
            assert_eq!(tree.contains_dir("/conflicted"), Ok(true));
        }
    }

    mod clear {
        use super::*;

        /// Test that clear() removes all child nodes from the root.
        #[test]
        fn test_clear_removes_all_children() {
            let mut tree = FileTree::new();

            // Add some directories and files
            tree.add_dir(Path::new("/home/user")).unwrap();
            tree.add_file(Path::new("/etc/config")).unwrap();

            // Verify that tree has children before clearing
            assert!(tree.root.childs.is_some());

            // Clear the tree
            tree.clear();

            // Verify that all children are removed
            assert!(tree.root.childs.is_none());
        }

        /// Test that root node itself is preserved after clear().
        #[test]
        fn test_clear_preserves_root_node() {
            let mut tree = FileTree::new();

            tree.add_dir(Path::new("/tmp")).unwrap();

            // Store root properties before clearing
            let root_name_before = tree.root.name.clone();

            tree.clear();

            // Verify root node is still there with original properties
            assert_eq!(tree.root.name, root_name_before);
            assert!(tree.root.childs.is_none()); // But children are cleared
        }

        /// Test clear() on empty tree — should be a no‑op.
        #[test]
        fn test_clear_on_empty_tree() {
            let mut tree = FileTree::new();

            // Tree is initially empty (no children)
            assert!(tree.root.childs.is_none());

            // Clear should not change anything
            tree.clear();

            assert!(tree.root.childs.is_none());
        }

        /// Test clearing tree with complex nested structure.
        #[test]
        fn test_clear_complex_structure() {
            let mut tree = FileTree::new();

            // Build a complex tree
            tree.add_dir(Path::new("/var/log")).unwrap();
            tree.add_dir(Path::new("/home/user/projects")).unwrap();
            tree.add_file(Path::new("/etc/passwd")).unwrap();
            tree.add_file(Path::new("/home/user/notes.txt")).unwrap();

            // Verify tree has content before clearing
            assert!(tree.contains_dir("/var/log").unwrap());
            assert!(tree.contains_dir("/home/user/projects").unwrap());
            assert!(tree.contains_file("/etc/passwd").unwrap());

            // Clear the tree
            tree.clear();

            // Verify all content is gone
            assert!(!tree.contains_dir("/var/log").unwrap_or(false));
            assert!(!tree.contains_dir("/home/user/projects").unwrap_or(false));
            assert!(!tree.contains_file("/etc/passwd").unwrap_or(false));
            assert!(tree.root.childs.is_none());
        }

        /// Test calling clear() multiple times — should remain empty.
        #[test]
        fn test_multiple_clear_calls() {
            let mut tree = FileTree::new();

            tree.add_dir(Path::new("/test")).unwrap();
            tree.clear(); // First clear
            tree.clear(); // Second clear

            assert!(tree.root.childs.is_none());
        }
    }
}
