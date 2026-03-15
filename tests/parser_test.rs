use nyaon::ast::node::Node;
use nyaon::lexer::lexer::Lexer;
use nyaon::parser::parser::Parser;
#[test]
fn test_parser_simple_object() {
    let input = r#"{ "lang": "nyaon", "fast": true }"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    if let Node::Object(map) = ast.root {
        assert_eq!(map.get("lang"), Some(&Node::String("nyaon".to_string())));
        assert_eq!(map.get("fast"), Some(&Node::Bool(true)));
    } else {
        panic!("Root should be parsed as an Object");
    }
}

#[test]
fn test_parser_array() {
    let input = r#"[ 1, null, "test" ]"#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    if let Node::Array(arr) = ast.root {
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Node::Number(1.0));
        assert_eq!(arr[1], Node::Null);
        assert_eq!(arr[2], Node::String("test".to_string()));
    } else {
        panic!("Root should be parsed as an Array");
    }
}
