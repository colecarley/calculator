pub mod interpreter;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod token;

use std::{env, fs};

use interpreter::interpreter::Interpreter;
use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let mut interpreter = Interpreter::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filename as an argument");
        return;
    }

    let filename = args[2].trim();
    let input = fs::read_to_string(filename).expect("Should have been able to read the file");

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens);
    let root = parser.parse();

    interpreter.evaluate(root);
}
