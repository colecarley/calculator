/*
Let -> Keyword Identifier '=' Expr

If -> Keyword Expr '{' Expr '}'
    | Keyword Expr '{' Expr '}' Else

Else -> Keyword '{' Expr '}'
    | Keyword If

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
        | identifier
        | Equality
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
                "if" => {
                    self.next();
                    self.if_statement(root);
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
        self.expression_tail(root, &term);
    }

    fn expression_tail(&mut self, root: &mut Node, first_term: &Node) {
        if (self.position) >= self.tokens.len() {
            root.children.push(first_term.clone());
            return;
        }
        if self.tokens[self.position].token_type == TokenType::Operator {
            match self.peek().value.as_str() {
                "+" | "-" => {
                    let mut operator = Node {
                        value: Some(self.peek().value.clone()),
                        node_type: NodeType::Operation,
                        children: Vec::new(),
                    };
                    operator.children.push(first_term.clone());

                    self.next();
                    let mut term = Node {
                        value: None,
                        node_type: NodeType::Term,
                        children: Vec::new(),
                    };
                    self.term(&mut term);
                    operator.children.push(term);
                    self.expression_tail(root, &mut operator);
                    // root.children.push(operator);
                }
                "==" | "!=" | ">" | ">=" | "<" | "<=" => {
                    let mut operator = Node {
                        value: Some(self.peek().value.clone()),
                        node_type: NodeType::Operation,
                        children: Vec::new(),
                    };

                    operator.children.push(first_term.clone());
                    self.next();
                    let mut term = Node {
                        value: None,
                        node_type: NodeType::Term,
                        children: Vec::new(),
                    };

                    self.term(&mut term);
                    operator.children.push(term);
                    self.expression_tail(root, &mut operator);
                    // root.children.push(operator);
                }
                _ => return,
            }
        } else {
            root.children.push(first_term.clone());
        }
    }

    fn term(&mut self, root: &mut Node) {
        let mut factor = Node {
            value: None,
            node_type: NodeType::Factor,
            children: Vec::new(),
        };
        self.factor(&mut factor);
        self.term_tail(root, &factor);
    }

    fn term_tail(&mut self, root: &mut Node, first_term: &Node) {
        if (self.position) >= self.tokens.len() {
            root.children.push(first_term.clone());
            return;
        }
        if self.peek().token_type == TokenType::Operator {
            match self.peek().value.as_str() {
                "*" | "/" => {
                    let mut operator = Node {
                        value: Some(self.peek().value.clone()),
                        node_type: NodeType::Operation,
                        children: Vec::new(),
                    };
                    operator.children.push(first_term.clone());

                    self.next();
                    let mut factor = Node {
                        value: None,
                        node_type: NodeType::Factor,
                        children: Vec::new(),
                    };
                    self.factor(&mut factor);
                    operator.children.push(factor);
                    self.term_tail(root, &mut operator);
                }
                _ => root.children.push(first_term.clone()),
            }
        } else {
            root.children.push(first_term.clone());
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

    fn if_statement(&mut self, root: &mut Node) {
        let mut if_statement = Node {
            value: Some("if".to_string()),
            node_type: NodeType::If,
            children: Vec::new(),
        };

        let mut expression = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.expression(&mut expression);

        if self.peek().token_type != TokenType::LeftBrace {
            panic!("Expected left brace");
        }

        self.next();

        let mut block = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.expression(&mut block);

        if self.peek().token_type != TokenType::RightBrace {
            panic!("Expected right brace");
        }

        self.next();
        if_statement.children.push(expression);
        if_statement.children.push(block);

        if self.peek().token_type == TokenType::Keyword {
            match self.peek().value.as_str() {
                "else" => {
                    self.next();
                    self.else_statement(&mut if_statement);
                }
                _ => panic!("Invalid keyword"),
            }
        }

        root.children.push(if_statement);
    }

    fn else_statement(&mut self, if_statement: &mut Node) {
        if self.peek().token_type != TokenType::LeftBrace {
            panic!("Expected left brace");
        }

        let mut else_block = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.next();
        self.expression(&mut else_block);

        if self.peek().token_type != TokenType::RightBrace {
            panic!("Expected right brace");
        }

        if_statement.children.push(else_block);
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
