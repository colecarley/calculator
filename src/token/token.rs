#[derive(PartialEq)]
pub enum TokenType {
    Number,
    Operator,
    LeftParen,
    RightParen,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token { token_type, value }
    }
}
