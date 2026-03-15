use crate::ast::node::Node;
use std::collections::HashMap;
pub struct NyaonDb {
    collections: HashMap<String, Node>,
}
impl Default for NyaonDb {
    fn default() -> Self {
        Self::new()
    }
}

impl NyaonDb {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
        }
    }
    pub fn insert_collection(&mut self, name: &str, data: Node) {
        self.collections.insert(name.to_string(), data);
    }
    pub fn get_collection(&self, name: &str) -> Option<&Node> {
        self.collections.get(name)
    }
    pub fn drop_collection(&mut self, name: &str) -> Option<Node> {
        self.collections.remove(name)
    }
}
