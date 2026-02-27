use std::path::{Component, Path};

use super::node::{Childs, ComponentType, Node};

pub struct FileTree {
    root: Node,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            root: Node::new("/", ComponentType::Dir),
        }
    }

    pub fn add_file(&mut self, path: &Path) -> Result<(), &'static str> {
        if !path.is_absolute() {
            return Err("Path must be absolute");
        }
        if path.as_os_str() == "/" {
            return Ok(());
        }

        let components: Vec<_> = path.components().collect();

        // Первый проход: создаём все необходимые директории
        let last_node = self.ensure_dirs(&components)?;

        // Второй проход: добавляем файл в последнюю директорию
        let last_component = components.last().ok_or("Empty path")?;
        let name = last_component.as_os_str();
        let childs = last_node
            .childs
            .get_or_insert_with(|| Box::new(Childs::new()));

        let node = Node::new(name.to_string_lossy().as_ref(), ComponentType::File);
        childs.files.insert(name.to_owned(), node);

        Ok(())
    }

    fn ensure_dirs(&mut self, components: &[Component<'_>]) -> Result<&mut Node, &'static str> {
        let mut current = &mut self.root;

        // Пропускаем RootDir и последний компонент (файл)
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

        Ok(current)
    }
}
