use nyaon::codegen::xml::XmlGenerator;
use nyaon::db::engine::NyaonDb;
use nyaon::lexer::scanner::Lexer;
use nyaon::parser::engine::Parser;

#[test]
fn test_real_world_scenario() {
    let real_input = r#"{
        "project": "nyaon_db",
        "settings": {
            "max_memory": 1024,
            "use_cache": true
        },
        "targets": ["cpp_cli", "rust_core"]
    }"#;
    let lexer = Lexer::new(real_input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    let xml = XmlGenerator::generate(&ast.root, "Config");
    assert!(xml.contains("<project>\n    nyaon_db\n  </project>"));
    assert!(xml.contains("<use_cache>\n      true\n    </use_cache>"));
    let mut db = NyaonDb::new();
    db.insert_collection("app_config", ast.root);
    assert!(db.get_collection("app_config").is_some());
    let dropped = db.drop_collection("app_config");
    assert!(dropped.is_some());
    assert!(db.get_collection("app_config").is_none());
}
