#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Colon,    // :
    Comma,    // ,
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    Illegal(char),
    EOF,
}
