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
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token { token_type, value }
    }
}
