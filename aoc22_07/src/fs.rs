use aoc_utils::tree::ArenaTree;

#[derive(Debug)]
pub enum FSError {
    DirectoryNotFound,
    AlreadyAtRoot,
}

#[derive(Debug, PartialEq)]
pub enum FSNode<'a> {
    File(&'a str, usize),
    Directory(&'a str),
}

impl<'a> FSNode<'a> {
    pub fn name(&self) -> &'a str {
        match self {
            FSNode::File(name, _) => *name,
            FSNode::Directory(name) => *name,
        }
    }
}

pub struct FileSystem<'a> {
    tree: ArenaTree<FSNode<'a>>,
    cwd: usize,
}

impl<'a> FileSystem<'a> {
    pub fn new() -> FileSystem<'a> {
        let mut tree = ArenaTree::new();
        let root_node = FSNode::Directory("/");
        tree.add_node(root_node);
        let cwd = 0;
        FileSystem { tree, cwd }
    }

    pub fn exists(&mut self, name: &str) -> bool {
        for child_idx in self
            .tree
            .get_node(self.cwd)
            .expect("cwd is always valid")
            .children()
        {
            let node = self
                .tree
                .get_node(*child_idx)
                .expect("child node idx is valid when iterating over .children");
            if node.value.name() == name {
                return true;
            }
        }
        false
    }

    pub fn cd_root(&mut self) {
        self.cwd = 0;
    }

    fn size(&self, idx: usize) -> usize {
        let node = self
            .tree
            .get_node(idx)
            .expect("Private function size not called without valid idx");
        match node.value {
            FSNode::File(_, size) => size,
            FSNode::Directory(_) => node
                .children()
                .iter()
                .map(|child_idx| self.size(*child_idx))
                .sum(),
        }
    }

    pub fn duh(&self) -> usize {
        self.size(self.cwd)
    }

    pub fn mkdir(&mut self, name: &'a str) {
        let dir = FSNode::Directory(name);
        self.tree
            .add_child_node(self.cwd, dir)
            .expect("cwd is always valid parent_idx");
    }

    pub fn touch(&mut self, name: &'a str, size: usize) {
        let file = FSNode::File(name, size);
        self.tree
            .add_child_node(self.cwd, file)
            .expect("cwd is always valid parent_idx");
    }

    pub fn cd(&mut self, name: &'a str) -> Result<(), FSError> {
        if name == ".." {
            let node = self
                .tree
                .get_node(self.cwd)
                .expect("cwd is always valid idx");
            match node.parent() {
                None => return Err(FSError::AlreadyAtRoot),
                Some(parent) => {
                    self.cwd = parent;
                    return Ok(());
                }
            }
        }
        for child_idx in self
            .tree
            .get_node(self.cwd)
            .expect("cwd is always valid")
            .children()
        {
            let node = self
                .tree
                .get_node(*child_idx)
                .expect("child node idx is valid when iterating over .children");
            if node.value.name() == name {
                self.cwd = node.idx();
                return Ok(());
            }
        }
        Err(FSError::DirectoryNotFound)
    }

    pub fn rget_all_dir_sizes(&self) -> Vec<usize> {
        let mut res = Vec::new();
        for idx in 0..self.tree.len() {
            match self
                .tree
                .get_node(idx)
                .expect(
                    "idx is valid when iterating over indicies less than the length of the tree",
                )
                .value
            {
                FSNode::File(_, _) => (),
                FSNode::Directory(_) => res.push(self.size(idx)),
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn filesystem_basic() {
        let mut fs = FileSystem::new();
        fs.mkdir("a");
        fs.mkdir("b");
        fs.cd("a").unwrap();
        fs.mkdir("d");
        fs.cd("d").unwrap();
        fs.touch("test.md", 600);
        fs.mkdir("e");
        fs.cd("e").unwrap();
        fs.touch("test.txt", 500);
        assert_eq!(fs.duh(), 500);
        fs.cd_root();
        assert_eq!(fs.duh(), 1100);
        assert_eq!(fs.rget_all_dir_sizes(), vec![1100, 1100, 0, 1100, 500]);
    }
}
