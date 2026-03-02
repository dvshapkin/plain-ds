use std::path::Path;

pub struct Iter<'a> {
    current: Option<&'a Path>
}

impl<'a> Iter<'a> {
    pub fn new(root: &'a Path) -> Self {
        Self {
            current: Some(root),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            None
        } else {
            todo!()
        }
    }
}
