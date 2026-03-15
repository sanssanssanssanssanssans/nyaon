use crate::ast::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct NyaonAst {
    pub root: Node,
}

impl NyaonAst {
    pub fn new(root: Node) -> Self {
        Self { root }
    }
}
