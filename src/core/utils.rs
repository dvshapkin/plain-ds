use std::path::Component;

pub fn path_comp_to_str<'a>(c: &'a Component) -> &'a str {
    c.as_os_str().to_str().expect("Path contains invalid UTF-8 characters")
}