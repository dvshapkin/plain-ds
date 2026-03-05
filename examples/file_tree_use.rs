use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use plain_ds::FileTree;

fn main() {
    tree();
    //empty_tree();
    //map();
    //empty_map();
    //set();
}

fn tree() {
    let mut tree = FileTree::new();
    let prefix = "/my/very/very/long/prefix/to/the/files";
    for i in 0..1000 {
        let file_name = format!("{}/file_{:04}", prefix, i);
        tree.add_file(file_name).unwrap();
    }
}

fn empty_tree() {
    let tree = FileTree::new();
    println!("{}", tree.is_empty())
}

fn set() {
    let mut set: BTreeSet<String> = BTreeSet::new();
    let prefix = "/my/very/very/long/prefix/to/the/files";
    for i in 0..1000 {
        let components = Path::new(prefix).components();
        assert_eq!(components.count(), 9);
        let file_name = format!("{}/file_{:04}", prefix, i);
        set.insert(file_name);
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

fn empty_map() {
    let map: BTreeMap<String, Option<&[u8]>> = BTreeMap::new();
    println!("{}", map.len())
}