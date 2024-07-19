use crate::token::token::Token;
use crate::token::token::TokenType;

#[derive(PartialEq, Debug)]
enum State {
    Start,
    Number,
    Operator,
    Whitespace,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Alpha,
    Comma,
    String,
    Comment,
}

pub struct Lexer<'a> {
    input: String,
    tokens: Vec<Token>,
    state: State,
    buffer: String,
    keywords: Vec<&'a str>,
    operators: Vec<&'a str>,
    current_line: i32,
}

impl Lexer<'_> {
    pub fn new<'a>(input: String) -> Lexer<'a> {
        Lexer {
            input,
            tokens: Vec::new(),
            state: State::Start,
            buffer: String::new(),
            keywords: vec![
                "let",
                "if",
                "else",
                "funk",
                "print",
                "println",
                "true",
                "false",
                "head",
                "tail",
                "len",
                "type",
                "is_bool",
                "is_number",
                "is_string",
                "is_list",
                "is_function",
                "input",
                "return",
                "bool",
                "int",
                "str",
                "list",
                "function",
            ],
            operators: vec![
                "+", "-", "*", "/", "%", "=", "==", ">=", "<=", ">", "<", "!=",
            ],
            current_line: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        use regex::Regex;
        let number = Regex::new(r"\d").unwrap();
        let operator = Regex::new(r"[+\-*/=><!%]").unwrap();
        let whitespace = Regex::new(r"\s").unwrap();
        let left_paren = Regex::new(r"\(").unwrap();
        let right_paren = Regex::new(r"\)").unwrap();
        let alpha = Regex::new(r"[a-zA-Z_]").unwrap();
        let left_brace = Regex::new(r"\{").unwrap();
        let right_brace = Regex::new(r"\}").unwrap();
        let left_bracket = Regex::new(r"\[").unwrap();
        let right_bracket = Regex::new(r"\]").unwrap();
        let comma: Regex = Regex::new(r",").unwrap();
        let string: Regex = Regex::new(r#"""#).unwrap();
        let semicolon = Regex::new(r";").unwrap();
        let newline = Regex::new(r"\n").unwrap();

        let input = self.input.clone();
        for (i, line) in input.lines().enumerate() {
            self.current_line = i as i32 + 1;
            for c in line.chars() {
                if self.state == State::String {
                    if string.is_match(&c.to_string()) {
                        self.buffer += &c.to_string();
                        self.push_string();
                        continue;
                    }
                }

                if self.buffer.ends_with("/") && c == '*' {
                    self.state = State::Comment;
                    self.buffer = String::new();
                    continue;
                }

                if self.state == State::Comment {
                    if self.buffer.ends_with("*") && c == '/' {
                        self.buffer = String::new();
                        self.state = State::Start;
                    } else {
                        self.buffer += &c.to_string();
                    }
                    continue;
                }

                if self.buffer.ends_with("/") && c == '/' {
                    self.state = State::Start;
                    self.buffer = String::new();
                    break;
                }

                if number.is_match(&c.to_string()) {
                    self.number(c);
                } else if alpha.is_match(&c.to_string()) {
                    self.alpha(c);
                } else if operator.is_match(&c.to_string()) {
                    self.operator(c);
                } else if right_paren.is_match(&c.to_string()) {
                    self.right_paren(c);
                } else if left_paren.is_match(&c.to_string()) {
                    self.left_paren(c);
                } else if whitespace.is_match(&c.to_string()) {
                    self.whitespace();
                } else if semicolon.is_match(&c.to_string()) {
                    self.whitespace();
                } else if newline.is_match(&c.to_string()) {
                    self.whitespace();
                } else if left_brace.is_match(&c.to_string()) {
                    self.left_brace(c);
                } else if right_brace.is_match(&c.to_string()) {
                    self.right_brace(c);
                } else if left_bracket.is_match(&c.to_string()) {
                    self.left_bracket(c);
                } else if right_bracket.is_match(&c.to_string()) {
                    self.right_bracket(c);
                } else if comma.is_match(&c.to_string()) {
                    self.comma(c);
                } else if string.is_match(&c.to_string()) {
                    self.string(c);
                } else {
                    panic!("Invalid character: {}", c);
                }
            }
        }

        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        if self.state == State::Operator {
            self.push_operator();
        }

        return self.tokens.clone();
    }

    fn number(&mut self, c: char) {
        if self.state == State::Alpha {
            return;
        }
        if self.state == State::Operator {
            self.push_operator();
        }

        self.state = State::Number;
        self.buffer += &c.to_string();
    }

    fn operator(&mut self, c: char) {
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        self.state = State::Operator;
        self.buffer += &c.to_string();
    }

    fn whitespace(&mut self) {
        if self.state == State::String {
            self.buffer += " ";
            return;
        }
        if self.state == State::Operator {
            self.push_operator();
        }
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }

        self.state = State::Whitespace;
    }

    fn left_paren(&mut self, c: char) {
        if self.state == State::Operator {
            self.push_operator();
        }
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        self.state = State::LeftParen;
        self.buffer += &c.to_string();
        self.push_left_paren();
    }

    fn right_paren(&mut self, c: char) {
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        self.state = State::RightParen;
        self.buffer += &c.to_string();
        self.push_right_paren();
    }

    fn left_brace(&mut self, c: char) {
        if self.state == State::Operator {
            self.push_operator();
        }
        if self.state == State::Number {
            self.push_number();
        }
        self.state = State::LeftBrace;
        self.buffer += &c.to_string();
        self.push_left_brace();
    }

    fn right_brace(&mut self, c: char) {
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        self.state = State::RightBrace;
        self.buffer += &c.to_string();
        self.push_right_brace();
    }

    fn left_bracket(&mut self, c: char) {
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        self.state = State::LeftBracket;
        self.buffer += &c.to_string();
        self.push_left_bracket();
    }

    fn right_bracket(&mut self, c: char) {
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        self.state = State::RightBracket;
        self.buffer += &c.to_string();
        self.push_right_bracket();
    }

    fn comma(&mut self, c: char) {
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        self.state = State::Comma;
        self.buffer += &c.to_string();
        self.push_comma();
    }

    fn alpha(&mut self, c: char) {
        if self.state == State::Operator {
            self.push_operator();
        }
        if self.state == State::Number {
            self.push_number();
        }
        if self.state == State::String {
            self.buffer += &c.to_string();
            return;
        }
        self.state = State::Alpha;
        self.buffer += &c.to_string();
    }

    fn string(&mut self, c: char) {
        if self.state == State::Alpha {
            self.push_alpha();
        }
        if self.state == State::Number {
            self.push_number();
        }
        if self.state == State::Operator {
            self.push_operator();
        }
        if self.state == State::String {
            self.buffer += &c.to_string();
            self.push_string();
            return;
        }
        self.state = State::String;
        self.buffer += &c.to_string();
    }

    fn push_alpha(&mut self) {
        if self.keywords.contains(&self.buffer.as_str()) {
            self.tokens.push(Token::new(
                TokenType::Keyword,
                self.buffer.clone(),
                self.current_line,
            ));
        } else {
            self.tokens.push(Token::new(
                TokenType::Identifier,
                self.buffer.clone(),
                self.current_line,
            ));
        }
        self.buffer = String::new();
    }

    fn push_number(&mut self) {
        self.tokens.push(Token::new(
            TokenType::Number,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_operator(&mut self) {
        if !self.operators.contains(&self.buffer.as_str()) {
            panic!("Invalid operator: {}", self.buffer,);
        }
        self.tokens.push(Token::new(
            TokenType::Operator,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_left_paren(&mut self) {
        self.tokens.push(Token::new(
            TokenType::LeftParen,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_right_paren(&mut self) {
        self.tokens.push(Token::new(
            TokenType::RightParen,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_left_brace(&mut self) {
        self.tokens.push(Token::new(
            TokenType::LeftBrace,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_right_brace(&mut self) {
        self.tokens.push(Token::new(
            TokenType::RightBrace,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_left_bracket(&mut self) {
        self.tokens.push(Token::new(
            TokenType::LeftBracket,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_right_bracket(&mut self) {
        self.tokens.push(Token::new(
            TokenType::RightBracket,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_comma(&mut self) {
        self.tokens.push(Token::new(
            TokenType::Comma,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
    }

    fn push_string(&mut self) {
        self.tokens.push(Token::new(
            TokenType::String,
            self.buffer.clone(),
            self.current_line,
        ));
        self.buffer = String::new();
        self.state = State::Start;
    }
}
