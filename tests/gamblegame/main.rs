use nyaon::ast::node::Node;
use nyaon::db::engine::NyaonDb;
use nyaon::lexer::lexer::Lexer;
use nyaon::parser::parser::Parser;
use std::fs;

#[test]
fn test_nyaon_gambling_simulation() {
    let file_path = "data.nyaon";
    let initial_data = r#"{
        "player": "nyaon_gambler",
        "money": 1000,
        "win_rate": 0.5
    }"#;
    fs::write(file_path, initial_data).expect("Failed to write data.nyaon");
    let input = fs::read_to_string(file_path).expect("Failed to read data.nyaon");
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    let mut db = NyaonDb::new();
    db.insert_collection("saves", ast.root);
    let bet_amount = 300.0;
    println!("\n[System] 게임을 시작합니다. 베팅 금액: {}", bet_amount);
    if let Some(Node::Object(map)) = db.get_collection("saves") {
        let mut current_money = match map.get("money") {
            Some(Node::Number(m)) => *m,
            _ => 0.0,
        };
        let win_rate = match map.get("win_rate") {
            Some(Node::Number(w)) => *w,
            _ => 0.0,
        };
        println!("[System] 현재 잔액: {}", current_money);
        let is_win = win_rate >= 0.5;
        if is_win {
            current_money += bet_amount;
            println!("[Result] 잭팟! {} 코인을 얻었습니다.", bet_amount);
        } else {
            current_money -= bet_amount;
            println!("[Result] 탕진... {} 코인을 잃었습니다.", bet_amount);
        }
        println!("[System] 최종 잔액: {}\n", current_money);
        let mut updated_map = map.clone();
        updated_map.insert("money".to_string(), Node::Number(current_money));
        db.insert_collection("saves", Node::Object(updated_map));
    } else {
        panic!("Saves collection is not an Object!");
    }
    if let Some(Node::Object(map)) = db.get_collection("saves") {
        assert_eq!(map.get("money"), Some(&Node::Number(1300.0)));
    }
    fs::remove_file(file_path).expect("Failed to clean up data.nyaon");
}
