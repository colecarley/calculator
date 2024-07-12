use crate::node::node::Node;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Number,
    Operator,
    LeftParen,
    RightParen,
    Keyword,
    Identifier,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    String,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(i32),
    String(String),
    Boolean(bool),
    Float(f32),
    List(Vec<Value>),
    Function(Node),
    Null,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: i32) -> Token {
        Token {
            token_type,
            value,
            line,
        }
    }
}
