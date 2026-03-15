use crate::lexer::scanner::Lexer;
use crate::lexer::token::Token;
pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
pub fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
pub fn read_string(lexer: &mut Lexer) -> Token {
    lexer.read_char();
    let start = lexer.position;
    while lexer.ch != '"' && lexer.ch != '\0' {
        lexer.read_char();
    }
    let end = lexer.position;
    let s: String = lexer.input[start..end].iter().collect();
    lexer.read_char();
    Token::String(s)
}
pub fn read_number(lexer: &mut Lexer) -> Token {
    let start = lexer.position;
    if lexer.ch == '-' {
        lexer.read_char();
    }
    while is_digit(lexer.ch) || lexer.ch == '.' {
        lexer.read_char();
    }
    let end = lexer.position;
    let s: String = lexer.input[start..end].iter().collect();
    let num = s.parse::<f64>().unwrap_or(0.0);
    Token::Number(num)
}
pub fn read_identifier(lexer: &mut Lexer) -> Token {
    let start = lexer.position;
    while is_letter(lexer.ch) {
        lexer.read_char();
    }
    let end = lexer.position;
    let s: String = lexer.input[start..end].iter().collect();
    match s.as_str() {
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "null" => Token::Null,
        _ => Token::Illegal(s.chars().next().unwrap_or('\0')),
    }
}
