use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub contents: String,
}

#[derive(Debug, Clone)]
pub struct Dir {
    pub name: String,
    pub children: HashMap<String, Node>, 
}

#[derive(Debug, Clone)]
pub enum Node {
    File(File),
    Dir(Dir),
}

#[derive(Debug, Clone)]
pub struct VirtualFs {
    pub root: Node,
}

impl VirtualFs {
    pub fn new() -> Self {
        Self{
            root: Node::Dir(Dir {
                name: "/".to_string(),
                children: HashMap::new(),
            }),
        }
    }
}
