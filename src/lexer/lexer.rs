use crate::token::token::Token;
use crate::token::token::TokenType;

#[derive(PartialEq)]
enum State {
    Start,
    Number,
    Operator,
    Whitespace,
    LeftParen,
    RightParen,
}

pub fn lex(input: String) -> Vec<Token> {
    use regex::Regex;
    let number = Regex::new(r"\d").unwrap();
    let operator = Regex::new(r"[+\-*/]").unwrap();
    let whitespace = Regex::new(r"\s").unwrap();
    let left_paren = Regex::new(r"\(").unwrap();
    let right_paren = Regex::new(r"\)").unwrap();

    let mut state = State::Start; // start, number, operator, whitespace, left_paren, right_paren
    let mut tokens = Vec::<Token>::new();

    let mut buffer = String::new();
    for c in input.chars() {
        if number.is_match(&c.to_string()) {
            state = State::Number;
            buffer += &c.to_string();
        } else if operator.is_match(&c.to_string()) {
            if state == State::Number {
                tokens.push(Token::new(TokenType::Number, buffer));
                buffer = String::new();
            }
            state = State::Operator;
            tokens.push(Token::new(TokenType::Operator, c.to_string()));
        } else if right_paren.is_match(&c.to_string()) {
            if state == State::Number {
                tokens.push(Token::new(TokenType::Number, buffer));
                buffer = String::new();
            }
            state = State::RightParen;
            tokens.push(Token::new(TokenType::RightParen, c.to_string()));
        } else if left_paren.is_match(&c.to_string()) {
            if state == State::Number {
                tokens.push(Token::new(TokenType::Number, buffer));
                buffer = String::new();
            }
            state = State::LeftParen;
            tokens.push(Token::new(TokenType::LeftParen, c.to_string()));
        } else if whitespace.is_match(&c.to_string()) {
            if state == State::Number {
                tokens.push(Token::new(TokenType::Number, buffer));
                buffer = String::new();
            }

            state = State::Whitespace;
        } else {
            println!("Invalid character: {}", c);
        }
    }

    return tokens;
}
