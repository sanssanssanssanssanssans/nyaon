use nyaon::lexer::scanner::Lexer;
use nyaon::lexer::token::Token;

#[test]
fn test_lexer_symbols_and_keywords() {
    let input = r#"{ } [ ] : , true false null"#;
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token(), Token::LBrace);
    assert_eq!(lexer.next_token(), Token::RBrace);
    assert_eq!(lexer.next_token(), Token::LBracket);
    assert_eq!(lexer.next_token(), Token::RBracket);
    assert_eq!(lexer.next_token(), Token::Colon);
    assert_eq!(lexer.next_token(), Token::Comma);
    assert_eq!(lexer.next_token(), Token::Bool(true));
    assert_eq!(lexer.next_token(), Token::Bool(false));
    assert_eq!(lexer.next_token(), Token::Null);
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_lexer_values() {
    let input = r#" "nyaon" -123.45 "#;
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token(), Token::String("nyaon".to_string()));
    assert_eq!(lexer.next_token(), Token::Number(-123.45));
    assert_eq!(lexer.next_token(), Token::EOF);
}
