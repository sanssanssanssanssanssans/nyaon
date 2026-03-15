use nyaon::ast::node::Node;
use nyaon::codegen::xml::XmlGenerator;
use std::collections::HashMap;
#[test]
fn test_xml_generation_from_ast() {
    let mut map = HashMap::new();
    map.insert(
        "player_name".to_string(),
        Node::String("nyaon_master".to_string()),
    );
    map.insert("is_banned".to_string(), Node::Bool(false));
    map.insert("score".to_string(), Node::Number(999.0));
    let root = Node::Object(map);
    let xml = XmlGenerator::generate(&root, "UserData");
    assert!(xml.contains("<UserData>"));
    assert!(xml.contains("</UserData>"));
    assert!(xml.contains("<player_name>\n    nyaon_master\n  </player_name>"));
    assert!(xml.contains("<is_banned>\n    false\n  </is_banned>"));
    assert!(xml.contains("<score>\n    999\n  </score>"));
}
