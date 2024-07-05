/*
Expr   -> Term ExprTail
ExprTail -> '+' Term ExprTail
         | '-' Term ExprTail
         | ε
Term   -> Factor TermTail
TermTail -> '*' Factor TermTail
         | '/' Factor TermTail
         | ε
Factor -> '(' Expr ')'
        | number
*/

use crate::node::node::Node;
use crate::token::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn next(&mut self) {
        self.position += 1;
    }

    pub fn print_tree(&self, root: &Node, level: usize) {
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", root.value);
        for child in &root.children {
            self.print_tree(child, level + 1);
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut root = Node {
            value: "Expr".to_string(),
            children: Vec::new(),
        };
        self.expression(&mut root);
        return root;
    }

    fn expression(&mut self, root: &mut Node) {
        println!("Expression");
        let mut term = Node {
            value: "Term".to_string(),
            children: Vec::new(),
        };
        self.term(&mut term);
        root.children.push(term);
        self.expression_tail(root);
    }

    fn expression_tail(&mut self, root: &mut Node) {
        println!("Expression Tail");
        if (self.position) >= self.tokens.len() {
            return;
        }
        if self.tokens[self.position].token_type == "operator" {
            match self.tokens[self.position].value.as_str() {
                "+" | "-" => {
                    root.value = self.tokens[self.position].value.clone();
                    self.next();
                    let mut term = Node {
                        value: "Term".to_string(),
                        children: Vec::new(),
                    };
                    self.term(&mut term);
                    root.children.push(term);
                    self.expression_tail(root);
                }
                _ => return,
            }
        } else {
            return;
        }
    }

    fn term(&mut self, root: &mut Node) {
        println!("Term");
        let mut factor = Node {
            value: "Factor".to_string(),
            children: Vec::new(),
        };
        self.factor(&mut factor);
        root.children.push(factor);
        self.term_tail(root);
    }

    fn term_tail(&mut self, root: &mut Node) {
        println!("Term Tail");
        if (self.position) >= self.tokens.len() {
            return;
        }
        if self.tokens[self.position].token_type == "operator" {
            match self.tokens[self.position].value.as_str() {
                "*" | "/" => {
                    root.value = self.tokens[self.position].value.clone();
                    self.next();
                    let mut factor = Node {
                        value: "Factor".to_string(),
                        children: Vec::new(),
                    };
                    self.factor(&mut factor);
                    root.children.push(factor);
                    self.term_tail(root);
                }
                _ => return,
            }
        } else {
            return;
        }
    }

    fn factor(&mut self, root: &mut Node) {
        println!("Factor");
        if self.tokens[self.position].token_type == "left_paren" {
            self.next();
            let mut expression = Node {
                value: "Expr".to_string(),
                children: Vec::new(),
            };
            self.expression(&mut expression);
            if self.tokens[self.position].token_type == "right_paren" {
                self.next();
                root.children.push(expression);
            } else {
                panic!("Expected right parenthesis");
            }
        } else if self.tokens[self.position].token_type == "number" {
            let node = Node {
                value: self.tokens[self.position].value.clone(),
                children: Vec::new(),
            };
            root.children.push(node);
            self.next();
        } else {
            panic!("Invalid factor");
        }
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }
}
