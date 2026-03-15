use crate::lexer::token::Token;
use crate::lexer::utils;
pub struct Lexer {
    pub(crate) input: Vec<char>,
    pub(crate) position: usize,
    pub(crate) read_position: usize,
    pub(crate) ch: char,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }
    pub(crate) fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.ch {
            '{' => {
                self.read_char();
                Token::LBrace
            }
            '}' => {
                self.read_char();
                Token::RBrace
            }
            '[' => {
                self.read_char();
                Token::LBracket
            }
            ']' => {
                self.read_char();
                Token::RBracket
            }
            ':' => {
                self.read_char();
                Token::Colon
            }
            ',' => {
                self.read_char();
                Token::Comma
            }
            '\0' => {
                self.read_char();
                Token::EOF
            }
            '"' => utils::read_string(self),
            _ => {
                if utils::is_digit(self.ch) || self.ch == '-' {
                    utils::read_number(self)
                } else if utils::is_letter(self.ch) {
                    utils::read_identifier(self)
                } else {
                    let illegal = Token::Illegal(self.ch);
                    self.read_char();
                    illegal
                }
            }
        }
    }
}
