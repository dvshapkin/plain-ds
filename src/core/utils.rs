use std::path::{Component, Path};

pub fn path_comp_to_str<'a>(c: &'a Component) -> &'a str {
    c.as_os_str().to_str().expect("Path contains invalid UTF-8 characters")
}

pub fn split_path(path: &Path) -> (Option<&Path>, Option<&str>) {
    let dir = path.parent();
    let file = path.file_name().map(|s| s.to_str().unwrap_or(""));
    (dir, file)
}