pub mod interpreter;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod token;

use interpreter::interpreter::Interpreter;
use lexer::lexer::lex;
use parser::parser::Parser;

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input;
}

fn main() {
    loop {
        let input = get_input();

        let vec = lex(input);

        let mut parser = Parser::new(vec);
        let root = parser.parse();

        parser.print_tree(&root, 0);

        let interpreter = Interpreter::new(root);

        let result = interpreter.evaluate();
        println!("Result: {}", result);
    }
}
