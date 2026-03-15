use nyaon::ast::node::Node;
use nyaon::codegen::xml::XmlGenerator;
use nyaon::db::engine::NyaonDb;
use nyaon::lexer::lexer::Lexer;
use nyaon::parser::parser::Parser;
use std::fs;

fn main() {
    let file_path = "data.nyaon";
    println!("===================================");
    println!("  Nyaon 카지노에 오신 것을 환영합니다! ");
    println!("===================================");
    let initial_data = r#"{
        "player": "nyaon_gambler",
        "money": 1000,
        "win_rate": 0.5
    }"#;
    fs::write(file_path, initial_data).expect("파일 생성 실패");
    let input = fs::read_to_string(file_path).unwrap();
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    let mut db = NyaonDb::new();
    db.insert_collection("saves", ast.root);
    let bet_amount = 300.0;
    println!("\n[System] 베팅 금액: {} 코인", bet_amount);
    if let Some(Node::Object(map)) = db.get_collection("saves") {
        let mut money = match map.get("money") {
            Some(Node::Number(m)) => *m,
            _ => 0.0,
        };
        println!("[System] 현재 잔액: {} 코인", money);
        let is_win = true;
        if is_win {
            money += bet_amount;
            println!("[Result] 잭팟! {} 코인을 얻었습니다!", bet_amount);
        } else {
            money -= bet_amount;
            println!("[Result] 탕진... {} 코인을 잃었습니다.", bet_amount);
        }
        println!("[System] 최종 잔액: {} 코인", money);
        let mut updated_map = map.clone();
        updated_map.insert("money".to_string(), Node::Number(money));
        db.insert_collection("saves", Node::Object(updated_map));
    }
    let final_node = db.get_collection("saves").unwrap();
    let xml = XmlGenerator::generate(final_node, "SaveFile");
    println!("\n=== 최종 저장 데이터 (XML) ===\n{}", xml);
    fs::remove_file(file_path).unwrap();
}
