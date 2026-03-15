use crate::ast::node::Node;
pub struct XmlGenerator;
impl XmlGenerator {
    pub fn generate(node: &Node, root_name: &str) -> String {
        let inner = Self::to_xml_string(node, 1);
        format!("<{}>\n{}</{}>", root_name, inner, root_name)
    }
    fn to_xml_string(node: &Node, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match node {
            Node::Null => format!("{}<null/>\n", pad),
            Node::Bool(b) => format!("{}{}\n", pad, b),
            Node::Number(n) => format!("{}{}\n", pad, n),
            Node::String(s) => format!("{}{}\n", pad, s),
            Node::Array(arr) => {
                let mut res = String::new();
                for item in arr {
                    res.push_str(&format!("{}<item>\n", pad));
                    res.push_str(&Self::to_xml_string(item, indent + 1));
                    res.push_str(&format!("{}</item>\n", pad));
                }
                res
            }
            Node::Object(obj) => {
                let mut res = String::new();
                for (key, val) in obj {
                    res.push_str(&format!("{}<{}>\n", pad, key));
                    res.push_str(&Self::to_xml_string(val, indent + 1));
                    res.push_str(&format!("{}</{}>\n", pad, key));
                }
                res
            }
        }
    }
}
