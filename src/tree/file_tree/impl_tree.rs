//! This module contains file-tree implementation.

use std::path::{Component, Path};

use super::node::DirNode;
use crate::{DSError, Result};

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
            root: DirNode::new(),
        }
    }

    /// Checks if the tree is empty.
    ///
    /// **Efficiency**: O(1)
    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
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
            return Err(DSError::NotFile {
                path: path.to_string_lossy().into_owned(),
            });
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
            return Err(DSError::NotFile {
                path: path.to_string_lossy().into_owned(),
            });
        }

        // Skip RootDir
        let mut components: Vec<_> = path.components().skip(1).collect();
        let file_component = components.pop().ok_or(DSError::EmptyPath)?;

        // First pass: create all necessary directories
        let parent_dir = self.ensure_dirs(&components);

        // Second pass: add the file to the last directory
        let name = file_component.as_os_str().to_string_lossy().to_string();
        parent_dir.files.insert(name);

        Ok(())
    }

    /// Removes a file from the tree.
    pub fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
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
            return Err(DSError::NotFile {
                path: path.to_string_lossy().into_owned(),
            });
        }

        // Skip RootDir
        let mut components: Vec<_> = path.components().skip(1).collect();
        let file_component = components.pop().ok_or(DSError::EmptyPath)?;

        // First pass: find the parent directory
        let parent = self.find_dir(&components)?;

        // Second pass: remove the file from the parent directory
        let file_name = file_component.as_os_str().to_string_lossy().to_string();

        if !parent.files.contains(&file_name) {
            // File doesn't exist — return error
            return Err(DSError::PathNotFound {
                path: path.to_string_lossy().into_owned(),
            });
        }
        parent.files.remove(&file_name);

        Ok(())
    }

    /// Removes a directory (with all its entries) from the tree.
    pub fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
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
            return Err(DSError::NotFile {
                path: path.to_string_lossy().into_owned(),
            });
        }

        // Skip RootDir
        let mut components: Vec<_> = path.components().skip(1).collect();
        let dir_component = components.pop().ok_or(DSError::EmptyPath)?;

        // First pass: find the parent directory
        let parent = self.find_dir(&components)?;

        // Second pass: remove the directory from the parent
        let dir_name = dir_component.as_os_str().to_string_lossy().to_string();

        if !parent.dirs.contains_key(&dir_name) {
            // Directory doesn't exist — return error
            return Err(DSError::PathNotFound {
                path: path.to_string_lossy().into_owned(),
            });
        }
        parent.dirs.remove(&dir_name);

        Ok(())
    }

    /// Clears all tree contents.
    ///
    /// **Efficiency**: O(1)
    pub fn clear(&mut self) {
        self.root.clear();
    }

    /// Visits all leaf elements in the tree and performs a `visitor` for each of them.
    pub fn visit(&self, mut visitor: impl FnMut(&Path)) {
        fn visit_recursive(parent: &Path, current: &DirNode, visitor: &mut impl FnMut(&Path)) {
            for name in current.files.iter() {
                visitor(parent.join(Path::new(name)).as_path())
            }
            for (name, sub_childs) in current.dirs.iter() {
                let parent = parent.join(Path::new(name));
                if sub_childs.is_empty() {
                    visitor(&parent);
                } else {
                    visit_recursive(&parent, sub_childs, visitor);
                }
            }
        }

        let parent = Path::new("/");

        visit_recursive(parent, &self.root, &mut visitor);
    }

    fn check_path(
        &self,
        components: &[Component<'_>],
        file_component: Option<Component<'_>>,
    ) -> bool {
        if self.is_empty() {
            return false;
        }

        let mut current = &self.root;
        let mut is_found = true;

        // First pass: checks all parent directories
        for component in components {
            let name = component.as_os_str().to_string_lossy().to_string();
            if !current.dirs.contains_key(&name) {
                is_found = false;
                break;
            }
            current = &current.dirs[&name];
        }

        // Second pass: checks the file in the last directory
        if let Some(file_component) = file_component
            && is_found
        {
            let name = file_component.as_os_str().to_string_lossy().to_string();
            if !current.files.contains(&name) {
                is_found = false;
            }
        }

        is_found
    }

    fn ensure_dirs(&mut self, components: &[Component<'_>]) -> &mut DirNode {
        let mut current = &mut self.root;

        for component in components {
            let name = component.as_os_str().to_string_lossy().to_string();

            if !current.dirs.contains_key(&name) {
                current.dirs.insert(name.clone(), DirNode::new());
            }
            current =  current.dirs.get_mut(&name).unwrap();  // safe unwrap
        }

        current
    }

    /// Helper method to find a directory node by path components.
    /// Returns error if any component in the path doesn't exist.
    fn find_dir(&mut self, components: &[Component<'_>]) -> Result<&mut DirNode> {
        let full_path = self.build_path(components);
        let mut current = &mut self.root;

        for component in components {
            let name = component.as_os_str().to_string_lossy().to_string();

            if let Some(dir) = current.dirs.get_mut(&name) {
                current = dir;
            } else {
                return Err(DSError::PathNotFound { path: full_path });
            }
        }

        Ok(current)
    }

    /// Helper method to build a string path from components for error reporting.
    fn build_path(&self, components: &[Component<'_>]) -> String {
        let mut path = String::from("/");
        for component in components {
            path.push_str(component.as_os_str().to_string_lossy().as_ref());
            path.push('/');
        }
        // Remove trailing slash if path is not root
        if path.len() > 1 {
            path.pop();
        }
        path
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
            assert!(tree.root.dirs.contains_key("home"));
        }

        /// Test adding nested directories.
        #[test]
        fn test_add_nested_dirs() {
            let mut tree = FileTree::new();

            // Add nested directories: /home/user/documents
            assert!(tree.add_dir(Path::new("/home/user/documents")).is_ok());

            // Verify the full path exists
            let home = tree.root.dirs.get("home").unwrap();
            let user = home.dirs.get("user").unwrap();
            assert!(user.dirs.contains_key("documents"));
        }

        /// Test adding directory with root path (should succeed without changes).
        #[test]
        fn test_add_root_dir() {
            let mut tree = FileTree::new();

            // Adding root directory should be a no‑op
            assert!(tree.add_dir(Path::new("/")).is_ok());

            // Root should still exist and have no children
            assert!(tree.root.is_empty());
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
            let home = tree.root.dirs.get("home").unwrap();
            let user = home.dirs.get("user").unwrap();
            assert!(user.files.contains("document.txt"));
        }

        /// Test adding file creates necessary intermediate directories.
        #[test]
        fn test_add_file_creates_intermediate_dirs() {
            let mut tree = FileTree::new();

            // Add file — this should create /projects/rust/ directories automatically
            assert!(tree.add_file(Path::new("/projects/rust/main.rs")).is_ok());

            // Verify full path was created
            let projects = tree
                .root
                .dirs
                .get("projects")
                .unwrap();
            let rust = projects.dirs.get("rust").unwrap();
            assert!(rust.files.contains("main.rs"));
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
            let tmp = tree.root.dirs.get("tmp").unwrap();
            let files = &tmp.files;
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
            let var = tree.root.dirs.get("var").unwrap();
            let log = var.dirs.get("log").unwrap();
            assert!(log.files.contains("system.log"));
        }

        /// Test handling of paths with special characters.
        #[test]
        fn test_add_path_with_special_chars() {
            let mut tree = FileTree::new();

            // Add directory with special characters
            assert!(tree.add_dir(Path::new("/special-@#$%/test")).is_ok());

            // Verify it was added correctly
            let special = tree
                .root
                .dirs
                .get("special-@#$%")
                .unwrap();
            assert!(special.dirs.contains_key("test"));
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
            assert_eq!(
                tree.contains_file("/"),
                Err(DSError::NotFile {
                    path: "/".to_string()
                })
            ); // Root can be considered as existing
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

    mod remove_file {
        use super::*;

        /// Test removing a simple file from root directory.
        #[test]
        fn test_remove_simple_file() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/document.txt")).unwrap();

            // Verify file exists before removal
            assert_eq!(tree.contains_file("/document.txt"), Ok(true));

            // Remove the file
            assert!(tree.remove_file("/document.txt").is_ok());

            // Verify file no longer exists
            assert_eq!(tree.contains_file("/document.txt"), Ok(false));
        }

        /// Test removing file from nested directory.
        #[test]
        fn test_remove_nested_file() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/home/user/document.txt")).unwrap();

            // Verify file exists
            assert_eq!(tree.contains_file("/home/user/document.txt"), Ok(true));

            // Remove the file
            assert!(tree.remove_file("/home/user/document.txt").is_ok());

            // Verify file is gone
            assert_eq!(tree.contains_file("/home/user/document.txt"), Ok(false));

            // But the directories should still exist
            assert_eq!(tree.contains_dir("/home"), Ok(true));
            assert_eq!(tree.contains_dir("/home/user"), Ok(true));
        }

        /// Test idempotent behavior — removing same file twice.
        #[test]
        fn test_idempotent_remove_file() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/tmp/file.txt")).unwrap();

            // First removal
            assert!(tree.remove_file("/tmp/file.txt").is_ok());
            // Second removal of same path
            assert!(tree.remove_file("/tmp/file.txt").is_err()); // Should not fail

            // File should not exist
            assert_eq!(tree.contains_file("/tmp/file.txt"), Ok(false));
        }

        /// Test removing non-existent file.
        #[test]
        fn test_remove_nonexistent_file() {
            let mut tree = FileTree::new();

            // Try to remove file that doesn't exist
            assert!(tree.remove_file("/nonexistent.txt").is_err());

            // Tree should be empty
            assert!(tree.root.is_empty());
        }

        /// Test error when removing empty path.
        #[test]
        fn test_remove_empty_path_error() {
            let mut tree = FileTree::new();

            let result = tree.remove_file("");
            assert!(result.is_err());
            if let Err(DSError::EmptyPath) = result {
                // Expected error type
            } else {
                panic!("Expected EmptyPath error for empty path");
            }
        }

        /// Test error when removing non-absolute path.
        #[test]
        fn test_remove_non_absolute_path_error() {
            let mut tree = FileTree::new();

            let result = tree.remove_file("relative/path/file.txt");
            assert!(result.is_err());
            if let Err(DSError::NotAbsolutePath { path }) = result {
                assert_eq!(path, "relative/path/file.txt");
            } else {
                panic!("Expected NotAbsolutePath error");
            }
        }

        /// Test error when trying to remove root as file.
        #[test]
        fn test_remove_root_as_file_error() {
            let mut tree = FileTree::new();

            let result = tree.remove_file("/");
            assert!(result.is_err());
            if let Err(DSError::NotFile { path }) = result {
                assert_eq!(path, "/");
            } else {
                panic!("Expected NotFile error for root path");
            }
        }

        /// Test removing multiple files from same directory.
        #[test]
        fn test_remove_multiple_files() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/tmp/file1.txt")).unwrap();
            tree.add_file(Path::new("/tmp/file2.txt")).unwrap();
            tree.add_file(Path::new("/tmp/script.sh")).unwrap();

            // Remove one file
            assert!(tree.remove_file("/tmp/file1.txt").is_ok());
            assert_eq!(tree.contains_file("/tmp/file1.txt"), Ok(false));

            // Remove another
            assert!(tree.remove_file("/tmp/script.sh").is_ok());
            assert_eq!(tree.contains_file("/tmp/script.sh"), Ok(false));

            // One file should still exist
            assert_eq!(tree.contains_file("/tmp/file2.txt"), Ok(true));
        }

        /// Test removing file with special characters in name.
        #[test]
        fn test_remove_file_with_special_chars() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/special-@#$%/test.file")).unwrap();

            assert_eq!(tree.contains_file("/special-@#$%/test.file"), Ok(true));

            // Remove file with special characters
            assert!(tree.remove_file("/special-@#$%/test.file").is_ok());

            assert_eq!(tree.contains_file("/special-@#$%/test.file"), Ok(false));
        }
    }

    mod remove_dir {
        use super::*;

        /// Test removing simple directory.
        #[test]
        fn test_remove_simple_dir() {
            let mut tree = FileTree::new();
            tree.add_dir(Path::new("/home")).unwrap();

            assert_eq!(tree.contains_dir("/home"), Ok(true));

            assert!(tree.remove_dir("/home").is_ok());

            assert_eq!(tree.contains_dir("/home"), Ok(false));
        }

        /// Test removing nested directory with files.
        #[test]
        fn test_remove_nested_dir_with_files() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/projects/rust/main.rs")).unwrap();
            tree.add_file(Path::new("/projects/python/script.py"))
                .unwrap();

            // Remove parent directory — this should remove all contents
            assert!(tree.remove_dir("/projects").is_ok());

            // All paths under /projects should be gone
            assert_eq!(tree.contains_dir("/projects"), Ok(false));
            assert_eq!(tree.contains_dir("/projects/rust"), Ok(false));
            assert_eq!(tree.contains_file("/projects/rust/main.rs"), Ok(false));
            assert_eq!(tree.contains_file("/projects/python/script.py"), Ok(false));
        }

        /// Test idempotent directory removal.
        #[test]
        fn test_idempotent_remove_dir() {
            let mut tree = FileTree::new();
            tree.add_dir(Path::new("/var/log")).unwrap();

            assert!(tree.remove_dir("/var/log").is_ok());
            assert!(tree.remove_dir("/var/log").is_err()); // Second removal

            assert_eq!(tree.contains_dir("/var/log"), Ok(false));
        }

        /// Test removing non-existent directory.
        #[test]
        fn test_remove_nonexistent_dir() {
            let mut tree = FileTree::new();

            // Removing non-existent directory should succeed (idempotent)
            assert!(tree.remove_dir("/nonexistent").is_err());
        }

        /// Test error when removing root directory.
        #[test]
        fn test_remove_root_dir_error() {
            let mut tree = FileTree::new();

            let result = tree.remove_dir("/");
            assert!(result.is_err());
            if let Err(DSError::NotFile { path }) = result {
                assert_eq!(path, "/");
            } else {
                panic!("Expected NotFile error for root path");
            }
        }

        /// Test removing directory with special characters in name.
        #[test]
        fn test_remove_dir_with_special_chars() {
            let mut tree = FileTree::new();
            tree.add_dir(Path::new("/special-@#$%/test")).unwrap();

            assert_eq!(tree.contains_dir("/special-@#$%/test"), Ok(true));

            // Remove directory with special characters
            assert!(tree.remove_dir("/special-@#$%/test").is_ok());

            assert_eq!(tree.contains_dir("/special-@#$%/test"), Ok(false));
        }

        /// Test error when removing file path as directory.
        #[test]
        fn test_remove_file_path_as_dir() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/tmp/document.txt")).unwrap();

            // Try to remove file path as directory
            assert!(tree.remove_dir("/tmp/document.txt").is_err());
            // Should error (idempotent) even though it's not a directory

            // File should still exist
            assert_eq!(tree.contains_file("/tmp/document.txt"), Ok(true));
        }

        /// Test removing intermediate directory — should remove all children.
        #[test]
        fn test_remove_intermediate_dir() {
            let mut tree = FileTree::new();
            tree.add_file(Path::new("/projects/rust/src/main.rs"))
                .unwrap();
            tree.add_file(Path::new("/projects/rust/tests/unit.rs"))
                .unwrap();
            tree.add_file(Path::new("/projects/python/app.py")).unwrap();

            // Remove intermediate directory /projects/rust
            assert!(tree.remove_dir("/projects/rust").is_ok());

            // All paths under /projects/rust should be gone
            assert_eq!(tree.contains_dir("/projects/rust"), Ok(false));
            assert_eq!(tree.contains_file("/projects/rust/src/main.rs"), Ok(false));
            assert_eq!(
                tree.contains_file("/projects/rust/tests/unit.rs"),
                Ok(false)
            );

            // But /projects/python should still exist
            assert_eq!(tree.contains_dir("/projects/python"), Ok(true));
            assert_eq!(tree.contains_file("/projects/python/app.py"), Ok(true));
        }

        /// Test removing directory that doesn't exist in middle of path.
        #[test]
        fn test_remove_nonexistent_middle_dir() {
            let mut tree = FileTree::new();
            tree.add_dir("/existing/path").unwrap();

            // Try to remove directory with non‑existent parent
            let result = tree.remove_dir("/nonexistent/parent/dir");
            assert!(result.is_err()); // Should be error

            // Existing path should still be there
            assert_eq!(tree.contains_dir("/existing/path"), Ok(true));
        }
    }

    mod find_dir {
        use super::*;

        /// Test successful finding of existing directory.
        #[test]
        fn test_find_existing_dir() {
            let mut tree = FileTree::new();
            tree.add_dir(Path::new("/home/user/documents")).unwrap();

            let components: Vec<_> = Path::new("/home/user").components().skip(1).collect();

            let result = tree.find_dir(&components);
            assert!(result.is_ok());
        }

        /// Test finding root directory.
        #[test]
        fn test_find_root_dir() {
            let mut tree = FileTree::new();

            let empty_components: Vec<Component<'_>> = vec![];
            let result = tree.find_dir(&empty_components);
            assert!(result.is_ok());
        }

        /// Test error when finding non-existent directory.
        #[test]
        fn test_find_nonexistent_dir() {
            let mut tree = FileTree::new();
            tree.add_dir(Path::new("/home")).unwrap();

            let components: Vec<_> = Path::new("/home/nonexistent")
                .components()
                .skip(1)
                .collect();

            let result = tree.find_dir(&components);
            assert!(result.is_err());
            if let Err(DSError::PathNotFound { path }) = result {
                assert_eq!(path, "/home/nonexistent");
            } else {
                panic!("Expected PathNotFound error");
            }
        }

        /// Test finding directory with partial path.
        #[test]
        fn test_find_partial_path() {
            let mut tree = FileTree::new();
            tree.add_dir(Path::new("/a/b/c/d")).unwrap();

            // Find /a/b
            let components: Vec<_> = Path::new("/a/b").components().skip(1).collect();

            let result = tree.find_dir(&components);
            assert!(result.is_ok());

            // Find /a/b/c
            let components2: Vec<_> = Path::new("/a/b/c").components().skip(1).collect();
            let result2 = tree.find_dir(&components2);
            assert!(result2.is_ok());
        }
    }

    mod build_path {
        use super::*;

        /// Test building path from components.
        #[test]
        fn test_build_path_simple() {
            let tree = FileTree::new();
            let components: Vec<_> = Path::new("/home/user").components().skip(1).collect();

            let path = tree.build_path(&components);
            assert_eq!(path, "/home/user");
        }

        /// Test building root path.
        #[test]
        fn test_build_root_path() {
            let tree = FileTree::new();
            let empty_components: Vec<Component<'_>> = vec![];

            let path = tree.build_path(&empty_components);
            assert_eq!(path, "/");
        }

        /// Test building complex path with special characters.
        #[test]
        fn test_build_path_special_chars() {
            let tree = FileTree::new();
            let components: Vec<_> = Path::new("/special-@#$%/test/path")
                .components()
                .skip(1)
                .collect();

            let path = tree.build_path(&components);
            assert_eq!(path, "/special-@#$%/test/path");
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
            assert!(!tree.root.is_empty());

            // Clear the tree
            tree.clear();

            // Verify that all children are removed
            assert!(tree.root.is_empty());
        }

        /// Test clear() on empty tree — should be a no‑op.
        #[test]
        fn test_clear_on_empty_tree() {
            let mut tree = FileTree::new();

            // Tree is initially empty (no children)
            assert!(tree.root.is_empty());

            // Clear should not change anything
            tree.clear();

            assert!(tree.root.is_empty());
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
            assert!(tree.root.is_empty());
        }

        /// Test calling clear() multiple times — should remain empty.
        #[test]
        fn test_multiple_clear_calls() {
            let mut tree = FileTree::new();

            tree.add_dir(Path::new("/test")).unwrap();
            tree.clear(); // First clear
            tree.clear(); // Second clear

            assert!(tree.root.is_empty());
        }
    }

    mod visit {
        use std::path::PathBuf;
        use super::*;

        /// Test visiting an empty tree — no paths should be visited.
        #[test]
        fn test_visit_empty_tree() {
            let tree = FileTree::new();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            assert!(visited.is_empty(), "Empty tree should not visit any paths");
        }

        /// Test visiting a tree with a single directory at the root level.
        /// Expected: visitor is called with "/home".
        #[test]
        fn test_visit_single_root_directory() {
            let mut tree = FileTree::new();
            tree.add_dir("/home").unwrap();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            assert_eq!(visited, vec![PathBuf::from("/home")], "Should visit /home");
        }

        /// Test visiting a tree with a single file at the root level.
        /// Expected: visitor is called with "/file.txt".
        #[test]
        fn test_visit_single_root_file() {
            let mut tree = FileTree::new();
            tree.add_file("/file.txt").unwrap();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            assert_eq!(visited, vec![PathBuf::from("/file.txt")], "Should visit file.txt");
        }

        /// Test visiting a nested directory structure.
        /// Tree: /home/user/documents
        /// Expected paths: "/home/user/documents".
        #[test]
        fn test_visit_nested_directories() {
            let mut tree = FileTree::new();
            tree.add_dir("/home/user/documents").unwrap();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            assert_eq!(
                visited,
                vec![PathBuf::from("/home/user/documents")],
                "Should visit the full nested directory path"
            );
        }

        /// Test visiting a tree with files in nested directories.
        /// Tree:
        ///   /etc/passwd
        ///   /var/log/messages
        /// Expected paths:
        ///   "/etc/passwd"
        ///   "/var/log/messages"
        #[test]
        fn test_visit_files_in_nested_directories() {
            let mut tree = FileTree::new();
            tree.add_file("/etc/passwd").unwrap();
            tree.add_file("/var/log/messages").unwrap();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            // BTreeSet гарантирует лексикографический порядок
            assert_eq!(
                visited,
                vec![
                    PathBuf::from("/etc/passwd"),
                    PathBuf::from("/var/log/messages")
                ],
                "Should visit all files with correct full paths"
            );
        }

        /// Test visiting a mixed structure with directories and files at different levels.
        /// Tree:
        ///   /home/user/document.txt
        ///   /tmp/script.sh
        ///   /var
        /// Expected paths:
        ///   "/home/user/document.txt"
        ///   "/tmp/script.sh"
        ///   "/var"
        #[test]
        fn test_visit_mixed_structure() {
            let mut tree = FileTree::new();
            tree.add_file("/home/user/document.txt").unwrap();
            tree.add_file("/tmp/script.sh").unwrap();
            tree.add_dir("/var").unwrap();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            // Ожидаемый порядок посещения: лексикографический по именам файлов и директорий
            assert_eq!(
                visited,
                vec![
                    PathBuf::from("/home/user/document.txt"),
                    PathBuf::from("/tmp/script.sh"),
                    PathBuf::from("/var")
                ],
                "Should visit all items with correct full paths in lexical order"
            );
        }

        /// Test that directories with children are not visited directly —
        /// only their leaf descendants are visited.
        /// Tree: /a/b/c (where c is empty)
        /// Expected: only "/a/b/c" is visited, not "/a" or "/a/b".
        #[test]
        fn test_visit_only_leaf_directories() {
            let mut tree = FileTree::new();
            tree.add_dir("/a/b/c").unwrap();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            assert_eq!(
                visited,
                vec![PathBuf::from("/a/b/c")],
                "Only leaf directories should be visited"
            );
        }

        /// Test visiting a complex tree with multiple branches and depths.
        /// Tree:
        ///   /projects/rust/main.rs
        ///   /projects/python/app.py
        ///   /docs/README.md
        ///   /temp
        /// Expected: all files and leaf directories with full paths.
        #[test]
        fn test_visit_complex_tree() {
            let mut tree = FileTree::new();
            tree.add_file("/projects/rust/main.rs").unwrap();
            tree.add_file("/projects/python/app.py").unwrap();
            tree.add_file("/docs/README.md").unwrap();
            tree.add_dir("/temp").unwrap();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            assert_eq!(
                visited,
                vec![
                    PathBuf::from("/docs/README.md"),
                    PathBuf::from("/projects/python/app.py"),
                    PathBuf::from("/projects/rust/main.rs"),
                    PathBuf::from("/temp")
                ],
                "Should visit all leaf items with full paths in correct order"
            );
        }

        /// Test visiting a tree where a directory contains both files and subdirectories.
        /// Tree:
        ///   /mixed/file1.txt
        ///   /mixed/subdir/file2.txt
        /// Expected:
        ///   "/mixed/file1.txt"
        ///   "/mixed/subdir/file2.txt"
        #[test]
        fn test_visit_directory_with_files_and_subdirs() {
            let mut tree = FileTree::new();
            tree.add_file("/mixed/file1.txt").unwrap();
            tree.add_file("/mixed/subdir/file2.txt").unwrap();
            let mut visited = Vec::new();
            tree.visit(|path| visited.push(path.to_path_buf()));
            assert_eq!(
                visited,
                vec![
                    PathBuf::from("/mixed/file1.txt"),
                    PathBuf::from("/mixed/subdir/file2.txt")
                ],
                "Should visit both files and files in subdirectories with full paths"
            );
        }
    }
}
