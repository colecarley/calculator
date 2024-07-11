/*

Program -> Statement Program
    | ε

Statement -> Let
    | If
    | Funk
    | Expr
    | FunctionCall

Let -> Keyword Identifier '=' Expr

If -> Keyword Expr '{' Expr '}'
    | Keyword Expr '{' Expr '}' Else

Else -> Keyword '{' Expr '}'
    | Keyword If

Funk -> Keyword Identifier '(' Args ')' '{' Expr '}'

Args -> Identifier ArgsTail
    | ε
ArgsTail -> ',' Identifier ArgsTail
    | ε

Parameters -> Expr ParametersTail
    | ε
ParametersTail -> ',' Expr ParametersTail
    | ε

FunctionCall -> Identifier '(' A ')'

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
        | functionCall
        | Equality
        | String
        | Boolean
        | Index
        | List

List -> '[' ListTail
ListTail -> Expr ListTailTail
ListTailTail -> ',' Expr ListTailTail
            | ']'


*/

use std::process::exit;

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

    fn prev(&mut self) {
        self.position -= 1;
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
            node_type: NodeType::Program,
            children: Vec::new(),
        };
        while self.position < self.tokens.len() {
            let mut expression = Node {
                value: None,
                node_type: NodeType::Expression,
                children: Vec::new(),
            };
            self.expression(&mut expression);
            root.children.push(expression);
        }
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
                "funk" => {
                    self.next();
                    self.function_declaration(root);
                    return;
                }
                "print" | "println" => {
                    self.function_call(root);
                    return;
                }
                "true" | "false" | "is_bool" | "is_number" | "is_string" | "is_list" | "type"
                | "head" | "tail" | "len" | "input" | "is_function" => {}
                _ => {
                    // panic!("Invalid keyword");
                    self.error(self.peek().clone(), "Invalid keyword");
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
        if self.peek().token_type == TokenType::Operator {
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
                "*" | "/" | "%" => {
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
                self.error(self.peek().clone(), "Expected right parenthesis");
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
            if (self.position + 1) < self.tokens.len() {
                self.next();
                if self.peek().token_type == TokenType::LeftParen {
                    self.prev();
                    self.function_call(root);
                } else if self.peek().token_type == TokenType::LeftBracket {
                    let mut node = Node {
                        value: None,
                        node_type: NodeType::Index,
                        children: Vec::new(),
                    };
                    self.prev();
                    self.index(&mut node);
                    root.children.push(node);
                } else {
                    self.prev();
                    let node = Node {
                        value: Some(self.peek().value.clone()),
                        node_type: NodeType::Identifier,
                        children: Vec::new(),
                    };
                    root.children.push(node);
                    self.next();
                }
            }
        } else if self.peek().token_type == TokenType::Keyword {
            match self.peek().value.as_str() {
                "true" | "false" => {
                    let node = Node {
                        value: Some(self.peek().value.clone()),
                        node_type: NodeType::Literal,
                        children: Vec::new(),
                    };
                    root.children.push(node);
                    self.next();
                }
                "is_bool" | "is_number" | "is_string" | "is_list" | "type" | "head" | "tail"
                | "len" | "input" | "is_function" => self.function_call(root),
                _ => self.error(self.peek().clone(), "Invalid keyword"),
            }
        } else if self.peek().token_type == TokenType::String {
            let node = Node {
                value: Some(self.peek().value.clone()),
                node_type: NodeType::Literal,
                children: Vec::new(),
            };
            root.children.push(node);
            self.next();
        } else if self.peek().token_type == TokenType::LeftBracket {
            let mut list = Node {
                value: None,
                node_type: NodeType::List,
                children: Vec::new(),
            };
            self.next();
            self.list(&mut list);
            root.children.push(list);
        } else {
            self.error(self.peek().clone(), "Invalid factor");
        }
    }

    fn assignment(&mut self, root: &mut Node) {
        let mut operation = Node {
            value: Some("=".to_string()),
            node_type: NodeType::Operation,
            children: Vec::new(),
        };

        if self.peek().token_type != TokenType::Identifier {
            self.error(self.peek().clone(), "Expected identifier");
        };

        let node = Node {
            value: Some(self.peek().value.clone()),
            node_type: NodeType::Identifier,
            children: Vec::new(),
        };
        operation.children.push(node);
        self.next();

        if self.peek().token_type != TokenType::Operator {
            self.error(self.peek().clone(), "Expected assignment operator");
        }

        if self.peek().value == "=".to_string() {
            self.next();
            let mut expression = Node {
                value: None,
                node_type: NodeType::Expression,
                children: Vec::new(),
            };
            self.expression(&mut expression);
            operation.children.push(expression);
            root.children.push(operation);
        } else {
            self.error(self.peek().clone(), "Expected assignment operator");
        }
    }

    fn list(&mut self, root: &mut Node) {
        let mut expression = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.expression(&mut expression);
        root.children.push(expression);
        self.list_tail(root);
    }

    fn list_tail(&mut self, root: &mut Node) {
        if self.peek().token_type == TokenType::RightBracket {
            self.next();
            return;
        }

        if self.peek().token_type != TokenType::Comma {
            self.error(self.peek().clone(), "Expected comma");
        }

        self.next();
        let mut expression = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.expression(&mut expression);
        root.children.push(expression);
        self.list_tail(root);
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
            self.error(self.peek().clone(), "Expected left brace");
        }

        self.next();

        let mut block = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.expression(&mut block);

        if self.peek().token_type != TokenType::RightBrace {
            self.error(self.peek().clone(), "Expected right brace");
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
                _ => self.error(self.peek().clone(), "Invalid keyword"),
            }
        }

        root.children.push(if_statement);
    }

    fn else_statement(&mut self, if_statement: &mut Node) {
        if self.peek().token_type != TokenType::LeftBrace {
            self.error(self.peek().clone(), "Expected left brace");
        }

        let mut else_block = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.next();
        self.expression(&mut else_block);

        if self.peek().token_type != TokenType::RightBrace {
            self.error(self.peek().clone(), "Expected right brace");
        }

        self.next();

        if_statement.children.push(else_block);
    }

    fn function_declaration(&mut self, root: &mut Node) {
        let mut operation = Node {
            value: Some("declaration".to_string()),
            node_type: NodeType::Operation,
            children: Vec::new(),
        };

        if self.peek().token_type != TokenType::Identifier {
            self.error(self.peek().clone(), "Expected identifier");
        }

        let identifier = Node {
            value: Some(self.peek().value.clone()),
            node_type: NodeType::Identifier,
            children: Vec::new(),
        };

        operation.children.push(identifier);
        self.next();

        let mut function = Node {
            value: None,
            node_type: NodeType::Function,
            children: Vec::new(),
        };

        let mut args = Node {
            value: None,
            node_type: NodeType::Args,
            children: Vec::new(),
        };

        if self.peek().token_type != TokenType::LeftParen {
            self.error(self.peek().clone(), "Expected left parenthesis");
        }

        self.next();
        if self.peek().token_type != TokenType::RightParen {
            self.args(&mut args);
        }

        if self.peek().token_type != TokenType::RightParen {
            self.error(self.peek().clone(), "Expected right parenthesis");
        }

        function.children.push(args);

        self.next();

        if self.peek().token_type != TokenType::LeftBrace {
            self.error(self.peek().clone(), "Expected left brace");
        }

        self.next();

        let mut block = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.expression(&mut block);

        if self.peek().token_type != TokenType::RightBrace {
            self.error(self.peek().clone(), "Expected right brace");
        }
        self.next();

        function.children.push(block);
        operation.children.push(function);
        root.children.push(operation);
    }

    fn args(&mut self, root: &mut Node) {
        if self.peek().token_type != TokenType::Identifier {
            self.error(self.peek().clone(), "Expected identifier");
        }

        let identifier = Node {
            value: Some(self.peek().value.clone()),
            node_type: NodeType::Identifier,
            children: Vec::new(),
        };

        root.children.push(identifier);

        self.next();
        self.args_tail(root);
    }

    fn args_tail(&mut self, root: &mut Node) {
        if self.peek().token_type != TokenType::Comma {
            return;
        }

        self.next();
        self.args(root);
    }

    fn function_call(&mut self, root: &mut Node) {
        let mut operation = Node {
            value: Some(self.peek().value.clone()),
            node_type: NodeType::Operation,
            children: Vec::new(),
        };

        self.next();

        if self.peek().token_type != TokenType::LeftParen {
            self.error(self.peek().clone(), "Expected left parenthesis");
        }

        let mut parameters = Node {
            value: None,
            node_type: NodeType::Parameters,
            children: Vec::new(),
        };

        self.next();
        self.parameters(&mut parameters);

        if self.peek().token_type != TokenType::RightParen {
            self.error(self.peek().clone(), "Expected right parenthesis");
        }

        self.next();

        operation.children.push(parameters);
        root.children.push(operation);
    }

    fn parameters(&mut self, root: &mut Node) {
        if self.peek().token_type == TokenType::RightParen {
            return;
        }

        let mut expression = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.expression(&mut expression);
        root.children.push(expression);
        self.parameters_tail(root);
    }

    fn parameters_tail(&mut self, root: &mut Node) {
        if self.peek().token_type != TokenType::Comma {
            return;
        }

        self.next();
        self.parameters(root);
    }

    fn index(&mut self, root: &mut Node) {
        if self.peek().token_type != TokenType::Identifier {
            self.error(self.peek().clone(), "Expected identifier");
        }

        let identifier = Node {
            value: Some(self.peek().value.clone()),
            node_type: NodeType::Identifier,
            children: Vec::new(),
        };

        root.children.push(identifier);

        self.next();

        if self.peek().token_type != TokenType::LeftBracket {
            self.error(self.peek().clone(), "Expected left bracket");
        }

        self.next();

        let mut expression = Node {
            value: None,
            node_type: NodeType::Expression,
            children: Vec::new(),
        };

        self.expression(&mut expression);
        root.children.push(expression);

        if self.peek().token_type != TokenType::RightBracket {
            self.error(self.peek().clone(), "Expected right bracket");
        }

        self.next();
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn error(&self, token: Token, message: &str) {
        eprintln!(
            "Error found near line {} with value '{}': {}",
            token.line, token.value, message
        );
        exit(1);
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }
}
