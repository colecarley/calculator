/*
Let -> Keyword Identifier '=' Expr

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

use crate::node::node::{Node, NodeType};
use crate::token::token::{Token, TokenType};

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

        print!("{:?}", root.node_type);
        root.value
            .is_some()
            .then(|| {
                print!(": {} \n", root.value.as_ref().unwrap());
            })
            .or_else(|| {
                print!("\n");
                Some(())
            });

        for child in &root.children {
            self.print_tree(child, level + 1);
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut root = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };
        self.expression(&mut root);
        return root;
    }

    fn expression(&mut self, root: &mut Node) {
        if self.peek().token_type == TokenType::Keyword {
            match self.peek().value.as_str() {
                "let" => {
                    self.next();
                    self.assignment(root);
                    return;
                }
                _ => {
                    panic!("Invalid keyword");
                }
            }
        }

        let mut term = Node {
            value: None,
            node_type: NodeType::Term,
            children: Vec::new(),
        };
        self.term(&mut term);
        root.children.push(term);
        self.expression_tail(root);
    }

    fn expression_tail(&mut self, root: &mut Node) {
        if (self.position) >= self.tokens.len() {
            return;
        }
        if self.tokens[self.position].token_type == TokenType::Operator {
            match self.peek().value.as_str() {
                "+" | "-" => {
                    root.value = Some(self.peek().value.clone());

                    self.next();
                    let mut term = Node {
                        value: None,
                        node_type: NodeType::Term,
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
        let mut factor = Node {
            value: None,
            node_type: NodeType::Factor,
            children: Vec::new(),
        };
        self.factor(&mut factor);
        root.children.push(factor);
        self.term_tail(root);
    }

    fn term_tail(&mut self, root: &mut Node) {
        if (self.position) >= self.tokens.len() {
            return;
        }
        if self.peek().token_type == TokenType::Operator {
            match self.peek().value.as_str() {
                "*" | "/" => {
                    root.value = Some(self.peek().value.clone());

                    self.next();
                    let mut factor = Node {
                        value: None,
                        node_type: NodeType::Factor,
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
        if self.peek().token_type == TokenType::LeftParen {
            self.next();
            let mut expression = Node {
                value: None,
                node_type: NodeType::Expression,
                children: Vec::new(),
            };
            self.expression(&mut expression);
            if self.peek().token_type == TokenType::RightParen {
                self.next();
                root.children.push(expression);
            } else {
                panic!("Expected right parenthesis");
            }
        } else if self.peek().token_type == TokenType::Number {
            let node = Node {
                value: Some(self.peek().value.clone()),
                node_type: NodeType::Literal,
                children: Vec::new(),
            };
            root.children.push(node);
            self.next();
        } else if self.peek().token_type == TokenType::Identifier {
            let node = Node {
                value: Some(self.peek().value.clone()),
                node_type: NodeType::Identifier,
                children: Vec::new(),
            };
            root.children.push(node);
            self.next();
        } else {
            panic!("Invalid factor");
        }
    }

    fn assignment(&mut self, root: &mut Node) {
        root.value = Some("=".to_string());

        if self.peek().token_type != TokenType::Identifier {
            panic!("Expected identifier");
        };

        let node = Node {
            value: Some(self.peek().value.clone()),
            node_type: NodeType::Identifier,
            children: Vec::new(),
        };
        root.children.push(node);
        self.next();

        if self.peek().token_type != TokenType::Operator {
            panic!("Expected assignment operator");
        }

        if self.peek().value == "=".to_string() {
            self.next();
            let mut expression = Node {
                value: None,
                node_type: NodeType::Expression,
                children: Vec::new(),
            };
            self.expression(&mut expression);
            root.children.push(expression);
        } else {
            panic!("Expected assignment operator");
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }
}
