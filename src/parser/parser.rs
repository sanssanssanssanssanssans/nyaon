use crate::ast::ast::NyaonAst;
use crate::ast::node::Node;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use std::collections::HashMap;
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}
impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            peek_token,
        }
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    pub fn parse(&mut self) -> NyaonAst {
        let root = self.parse_value();
        NyaonAst::new(root)
    }
    fn parse_value(&mut self) -> Node {
        match &self.current_token {
            Token::LBrace => self.parse_object(),
            Token::LBracket => self.parse_array(),
            Token::String(s) => {
                let node = Node::String(s.clone());
                self.next_token();
                node
            }
            Token::Number(n) => {
                let node = Node::Number(*n);
                self.next_token();
                node
            }
            Token::Bool(b) => {
                let node = Node::Bool(*b);
                self.next_token();
                node
            }
            Token::Null => {
                self.next_token();
                Node::Null
            }
            _ => {
                self.next_token();
                Node::Null
            }
        }
    }
    fn parse_object(&mut self) -> Node {
        let mut map = HashMap::new();
        self.next_token();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            let key = if let Token::String(k) = &self.current_token {
                k.clone()
            } else {
                "".to_string()
            };
            self.next_token();
            if self.current_token == Token::Colon {
                self.next_token();
            }
            let value = self.parse_value();
            map.insert(key, value);
            if self.current_token == Token::Comma {
                self.next_token();
            }
        }
        self.next_token();
        Node::Object(map)
    }
    fn parse_array(&mut self) -> Node {
        let mut elements = Vec::new();
        self.next_token();
        while self.current_token != Token::RBracket && self.current_token != Token::EOF {
            let value = self.parse_value();
            elements.push(value);
            if self.current_token == Token::Comma {
                self.next_token();
            }
        }
        self.next_token();
        Node::Array(elements)
    }
}
