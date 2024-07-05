use crate::token::token::Token;

pub fn lex(input: String) -> Vec<Token> {
    use regex::Regex;
    let number = Regex::new(r"\d").unwrap();
    let operator = Regex::new(r"[+\-*/]").unwrap();
    let whitespace = Regex::new(r"\s").unwrap();
    let left_paren = Regex::new(r"\(").unwrap();
    let right_paren = Regex::new(r"\)").unwrap();

    let mut state = "start"; // start, number, operator, whitespace, left_paren, right_paren
    let mut tokens = Vec::<Token>::new();

    let mut buffer = String::new();
    for c in input.chars() {
        if number.is_match(&c.to_string()) {
            state = "number";
            buffer += &c.to_string();
        } else if operator.is_match(&c.to_string()) {
            if state == "number" {
                tokens.push(Token::new("number".to_string(), buffer));
                buffer = String::new();
            }
            state = "operator";
            tokens.push(Token::new("operator".to_string(), c.to_string()));
        } else if right_paren.is_match(&c.to_string()) {
            if state == "number" {
                tokens.push(Token::new("number".to_string(), buffer));
                buffer = String::new();
            }
            state = "right_paren";
            tokens.push(Token::new("right_paren".to_string(), c.to_string()));
        } else if left_paren.is_match(&c.to_string()) {
            if state == "number" {
                tokens.push(Token::new("number".to_string(), buffer));
                buffer = String::new();
            }
            state = "left_paren";
            tokens.push(Token::new("left_paren".to_string(), c.to_string()));
        } else if whitespace.is_match(&c.to_string()) {
            if state == "number" {
                tokens.push(Token::new("number".to_string(), buffer));
                buffer = String::new();
            }

            state = "whitespace";
        } else {
            println!("Invalid character: {}", c);
        }
    }

    return tokens;
}
