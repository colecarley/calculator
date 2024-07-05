pub struct Token {
    pub token_type: String,
    pub value: String,
}

impl Token {
    pub fn new(token_type: String, value: String) -> Token {
        Token { token_type, value }
    }
}
