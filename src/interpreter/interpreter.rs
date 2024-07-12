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
                    "println" => {
                        let value = self.evaluate_helper(&root.children[0]);
                        self.print_value(&value);
                        println!();
                        return value;
                    }
                    "head" => {
                        let list = self.evaluate_helper(&root.children[0]);
                        if let Value::List(list) = list {
                            return list[0].clone();
                        } else {
                            panic!("Expected a list");
                        }
                    }
                    "tail" => {
                        let list = self.evaluate_helper(&root.children[0]);
                        if let Value::List(list) = list {
                            return Value::List(list[1..].to_vec().clone());
                        } else if let Value::String(string) = list {
                            return Value::String(string[1..].to_string());
                        } else {
                            panic!("Expected a list or a string");
                        }
                    }
                    "len" => {
                        let list = self.evaluate_helper(&root.children[0]);
                        if let Value::List(list) = list {
                            return Value::Number(list.len() as i32);
                        } else if let Value::String(string) = list {
                            return Value::Number(string.len() as i32);
                        } else {
                            panic!("Expected a list or a string");
                        }
                    }
                    "type" => {
                        let value = self.evaluate_helper(&root.children[0]);
                        return match value {
                            Value::Number(_) => Value::String("number".to_string()),
                            Value::Float(_) => Value::String("float".to_string()),
                            Value::String(_) => Value::String("string".to_string()),
                            Value::Boolean(_) => Value::String("bool".to_string()),
                            Value::List(_) => Value::String("list".to_string()),
                            Value::Null => Value::String("null".to_string()),
                        };
                    }
                    "is_bool" => {
                        let value = self.evaluate_helper(&root.children[0]);
                        return match value {
                            Value::Boolean(_) => Value::Boolean(true),
                            _ => Value::Boolean(false),
                        };
                    }
                    "is_number" => {
                        let value = self.evaluate_helper(&root.children[0]);
                        return match value {
                            Value::Number(_) => Value::Boolean(true),
                            _ => Value::Boolean(false),
                        };
                    }
                    "is_string" => {
                        let value = self.evaluate_helper(&root.children[0]);
                        return match value {
                            Value::String(_) => Value::Boolean(true),
                            _ => Value::Boolean(false),
                        };
                    }
                    "is_list" => {
                        let value = self.evaluate_helper(&root.children[0]);
                        return match value {
                            Value::List(_) => Value::Boolean(true),
                            _ => Value::Boolean(false),
                        };
                    }
                    "is_function" => {
                        return match self.functions.contains_key(val) {
                            true => Value::Boolean(true),
                            false => Value::Boolean(false),
                        };
                    }
                    "input" => {
                        let mut input = String::new();
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");
                        return Value::String(input.trim().to_string());
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
        if root.node_type == NodeType::Block {
            for child in root.children.iter().take(root.children.len() - 1) {
                self.evaluate_helper(child);
            }
            return self.evaluate_helper(root.children.last().expect("expected a child"));
        }

        if root.node_type == NodeType::List {
            let values: Vec<Value> = root
                .children
                .iter()
                .map(|child| self.evaluate_helper(child))
                .collect();

            return Value::List(values);
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
                if root.children.len() >= 2 {
                    return self.evaluate_helper(&root.children[1]);
                } else {
                    return Value::Null;
                }
            } else {
                if root.children.len() >= 3 {
                    return self.evaluate_helper(&root.children[2]);
                } else {
                    return Value::Null;
                }
            }
        }

        if root.node_type == NodeType::Index {
            let indexable_name = root.children[0]
                .value
                .as_ref()
                .expect("expected an identifier for value");
            let index = self.evaluate_helper(&root.children[1]);
            let index = if let Value::Number(index) = index {
                index
            } else {
                panic!("Expected a number");
            };

            let indexable = self
                .identifiers
                .get(indexable_name)
                .expect("expected an known identifier");
            if let Value::String(string) = indexable {
                return Value::String(
                    (string.as_bytes()[index as usize].clone() as char).to_string(),
                );
            }
            if let Value::List(list) = indexable {
                return list[index as usize].clone();
            } else {
                panic!("Expected a string or a list");
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
                } else if let Value::List(first) = &values[0] {
                    if let Value::List(second) = &values[1] {
                        let mut result = first.clone();
                        result.extend(second.clone());
                        return Value::List(result);
                    } else {
                        panic!("Expected a list");
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
            Value::Number(val) => print!("{}", val),
            Value::Float(val) => print!("{}", val),
            Value::String(val) => print!("{}", val),
            Value::Boolean(val) => print!("{}", val),
            Value::Null => print!("null"),
            Value::List(val) => {
                print!("[");
                for (i, v) in val.iter().enumerate() {
                    self.print_value(v);
                    if i < val.len() - 1 {
                        print!(", ");
                    }
                }
                print!("]");
            }
        }
    }
}
