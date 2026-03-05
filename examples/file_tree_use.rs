use std::collections::BTreeMap;
use plain_ds::FileTree;

fn main() {
    tree();
    //map();
}

fn tree() {
    let mut tree = FileTree::new();
    let prefix = "/my/very/very/long/prefix/to/the/files";
    for i in 0..1000 {
        let file_name = format!("{}/file_{:04}", prefix, i);
        tree.add_file(file_name).unwrap();
    }
}

fn map() {
    let mut set: BTreeMap<String, Option<&[u8]>> = BTreeMap::new();
    let prefix = "/my/very/very/long/prefix/to/the/files";
    for i in 0..1000 {
        let file_name = format!("{}/file_{:04}", prefix, i);
        set.insert(file_name, None);
    }
}