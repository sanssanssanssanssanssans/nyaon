use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Node>),
    Object(HashMap<String, Node>),
}
