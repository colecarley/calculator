use core::panic;
use std::collections::HashMap;

use crate::{
    node::node::{Node, NodeType},
    token::token::Value,
};

pub struct Interpreter {
    identifiers: HashMap<String, Value>,
    functions: HashMap<String, Node>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            identifiers: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn new_with_identifiers_and_functions(
        identifiers: HashMap<String, Value>,
        functions: HashMap<String, Node>,
    ) -> Interpreter {
        Interpreter {
            identifiers,
            functions,
        }
    }

    pub fn evaluate(&mut self, root: Node) -> Value {
        let mut result = Value::Number(0);
        for child in &root.children {
            result = self.evaluate_helper(child);
        }
        result
    }

    fn evaluate_helper(&mut self, root: &Node) -> Value {
        if root.node_type == NodeType::Operation {
            match &root.value {
                Some(val) => match val.clone().as_str() {
                    "=" => {
                        let identifier = root.children[0]
                            .value
                            .as_ref()
                            .expect("expected an identifier");
                        let value = self.evaluate_helper(&root.children[1]);
                        self.identifiers.insert(identifier.clone(), value.clone());
                        return value;
                    }
                    "declaration" => {
                        let identifier = root.children[0]
                            .value
                            .as_ref()
                            .expect("expected an identifier");
                        self.functions
                            .insert(identifier.clone(), root.children[1].clone());
                        return Value::Number(0);
                    }
                    "print" => {
                        let value = self.evaluate_helper(&root.children[0]);
                        self.print_value(&value);
                        return value;
                    }
                    _ => {
                        if self.functions.contains_key(val) {
                            let values: Vec<Value> = root.children[0]
                                .children
                                .iter()
                                .map(|child| self.evaluate_helper(child))
                                .collect();

                            return self.evaluate_function(val.clone(), values);
                        }
                    }
                },
                None => {}
            }
        }

        if root.children.len() == 0 {
            return self.parse_value(root);
        }

        if root.children.len() == 1 {
            return self.evaluate_helper(&root.children[0]);
        }

        if root.node_type == NodeType::If {
            let condition = self.evaluate_helper(&root.children[0]);
            let condition = match condition {
                Value::Boolean(val) => val,
                _ => panic!("Expected a boolean"),
            };

            if condition {
                return self.evaluate_helper(&root.children[1]);
            } else {
                return self.evaluate_helper(&root.children[2]);
            }
        }

        if root.node_type == NodeType::Index {
            let list_name = root.children[0]
                .value
                .as_ref()
                .expect("expected an identifier for value");
            let index = self.evaluate_helper(&root.children[1]);
            let index = if let Value::Number(index) = index {
                index
            } else {
                panic!("Expected a number");
            };

            let string = self.identifiers.get(list_name).unwrap();
            if let Value::String(string) = string {
                return Value::String(
                    (string.as_bytes()[index as usize].clone() as char).to_string(),
                );
            } else {
                panic!("Expected a list");
            }
        }

        let values: Vec<Value> = root
            .children
            .iter()
            .map(|child| self.evaluate_helper(child))
            .collect();

        match root.value.as_ref().unwrap().as_str() {
            "+" => {
                if let Value::Number(first) = &values[0] {
                    if let Value::Number(second) = &values[1] {
                        return Value::Number(first + second);
                    } else {
                        panic!("Expected a number");
                    }
                } else if let Value::String(first) = &values[0] {
                    if let Value::String(second) = &values[1] {
                        return Value::String(format!("{}{}", first, second));
                    } else {
                        panic!("Expected a string");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            "-" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Number(first - second);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            "*" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Number(first * second);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            "/" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Number(first / second);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            "%" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Number(first % second);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            "==" => {
                if let Value::Number(first) = &values[0] {
                    if let Value::Number(second) = &values[1] {
                        return Value::Boolean(first == second);
                    } else {
                        panic!("Expected a number");
                    }
                } else if let Value::String(first) = &values[0] {
                    if let Value::String(second) = &values[1] {
                        return Value::Boolean(first == second);
                    } else {
                        panic!("Expected a string");
                    }
                } else if let Value::Boolean(first) = &values[0] {
                    if let Value::Boolean(second) = &values[1] {
                        return Value::Boolean(first == second);
                    } else {
                        panic!("Expected a boolean");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            "!=" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Boolean(first != second);
                    } else {
                        panic!("Expected a number");
                    }
                } else if let Value::String(first) = &values[0] {
                    if let Value::String(second) = &values[1] {
                        return Value::Boolean(first != second);
                    } else {
                        panic!("Expected a string");
                    }
                } else if let Value::Boolean(first) = &values[0] {
                    if let Value::Boolean(second) = &values[1] {
                        return Value::Boolean(first != second);
                    } else {
                        panic!("Expected a boolean");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            ">" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Boolean(first > second);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            ">=" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Boolean(first >= second);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            "<" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Boolean(first < second);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            "<=" => {
                if let Value::Number(first) = values[0] {
                    if let Value::Number(second) = values[1] {
                        return Value::Boolean(first <= second);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a number");
                }
            }
            _ => panic!("Invalid operator"),
        }
    }

    fn evaluate_function(&mut self, function_name: String, parameter_values: Vec<Value>) -> Value {
        let function = self.functions.get(&function_name).unwrap();

        let arg_names: Vec<String> = function.children[0]
            .children
            .iter()
            .map(|child| {
                child
                    .value
                    .as_ref()
                    .expect("expected an identifier for value")
                    .clone()
            })
            .collect();

        let mut arg_values: HashMap<String, Value> = HashMap::new();
        for (i, arg_name) in arg_names.iter().enumerate() {
            arg_values.insert(arg_name.clone(), parameter_values[i].clone());
        }

        let mut interpreter =
            Interpreter::new_with_identifiers_and_functions(arg_values, self.functions.clone());

        let result = interpreter.evaluate(function.children[1].clone());

        result
    }

    fn parse_value(&self, node: &Node) -> Value {
        if node.node_type == NodeType::Literal {
            let value = node.value.as_ref().unwrap();
            if value.contains("\"") {
                return Value::String(value.clone().replace("\"", ""));
            } else if value.contains(".") {
                return Value::Float(value.parse().unwrap());
            } else if value == "true" {
                return Value::Boolean(true);
            } else if value == "false" {
                return Value::Boolean(false);
            } else {
                return Value::Number(value.parse().unwrap());
            }
        } else if node.node_type == NodeType::Identifier {
            return self
                .identifiers
                .get(
                    node.value
                        .as_ref()
                        .expect("expected an identifier for value"),
                )
                .unwrap()
                .clone();
        } else {
            panic!("Invalid value");
        }
    }

    fn print_value(&self, value: &Value) {
        match value {
            Value::Number(val) => println!("{}", val),
            Value::Float(val) => println!("{}", val),
            Value::String(val) => println!("\"{}\"", val),
            Value::Boolean(val) => println!("{}", val),
            _ => panic!("Invalid value"),
        }
    }
}
