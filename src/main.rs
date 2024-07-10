pub mod interpreter;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod token;

use interpreter::interpreter::Interpreter;
use lexer::lexer::Lexer;
use parser::parser::Parser;

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input;
}

fn main() {
    let mut interpreter = Interpreter::new();
    loop {
        let input = get_input();

        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex();

        let mut parser = Parser::new(tokens);
        let root = parser.parse();
        parser.print_tree(&root, 0);

        // let result =
        interpreter.evaluate(root);
        // println!("{}", result);
    }
}
