use core::panic;
use std::collections::HashMap;

use crate::{
    node::node::{Node, NodeType},
    token::token::Value,
};

struct ScopeManager {
    scopes: Vec<Box<HashMap<String, Value>>>,
}

impl ScopeManager {
    fn new() -> ScopeManager {
        let scopes = vec![Box::new(HashMap::new())];
        ScopeManager { scopes }
    }

    fn insert_identifier(&mut self, identifier: String, value: Value) {
        let top = self.scopes.last().unwrap();
        let mut new_top = top.clone();
        new_top.insert(identifier, value);
        self.scopes.pop();
        self.scopes.push(new_top);
    }

    fn get_identifier(&self, identifier: &str) -> Value {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(identifier) {
                return scope.get(identifier).unwrap().clone();
            }
        }
        panic!("Identifier not found");
    }

    fn contains_identifier(&self, identifier: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(identifier) {
                return true;
            }
        }
        return false;
    }

    fn new_scope_with_values(&mut self, values: HashMap<String, Value>) {
        self.new_scope();
        for (key, value) in values {
            self.insert_identifier(key, value);
        }
    }

    fn new_scope(&mut self) {
        let top = self.scopes.last().unwrap();
        let new_top = top.clone();
        self.scopes.push(new_top);
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

pub struct Interpreter {
    scope_manager: ScopeManager,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let scope_manager = ScopeManager::new();
        Interpreter { scope_manager }
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
                        self.scope_manager
                            .insert_identifier(identifier.clone(), value.clone());
                        return value;
                    }
                    "declaration" => {
                        let identifier = root.children[0]
                            .value
                            .as_ref()
                            .expect("expected an identifier");

                        self.scope_manager.insert_identifier(
                            identifier.clone(),
                            Value::Function(root.children[1].clone()),
                        );

                        return Value::Function(root.children[1].clone());
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
                            Value::Function(_) => Value::String("function".to_string()),
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
                        return match self.scope_manager.contains_identifier(val) {
                            true => {
                                let val = self.scope_manager.get_identifier(val);
                                if let Value::Function(_) = val {
                                    Value::Boolean(true)
                                } else {
                                    Value::Boolean(false)
                                }
                            }
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
                        if self.scope_manager.contains_identifier(val) {
                            let values: Vec<Value> = root.children[0]
                                .children
                                .iter()
                                .map(|child| self.evaluate_helper(child))
                                .collect();
                            let function = self.scope_manager.get_identifier(val).clone();
                            return self.evaluate_function(&function, values);
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

            let indexable = self.scope_manager.get_identifier(indexable_name);
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

    fn evaluate_function(&mut self, function: &Value, parameter_values: Vec<Value>) -> Value {
        if let Value::Function(function) = function {
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

            let mut arg_values = HashMap::new();
            for (i, arg_name) in arg_names.iter().enumerate() {
                arg_values.insert(arg_name.clone(), parameter_values[i].clone());
            }

            self.scope_manager.new_scope_with_values(arg_values);
            for child in function.children[1]
                .children
                .iter()
                .take(function.children[1].children.len() - 1)
            {
                self.evaluate_helper(child);
            }
            let result = self.evaluate_helper(
                function.children[1]
                    .children
                    .last()
                    .expect("expected a child"),
            );

            self.scope_manager.pop_scope();

            return result;
        } else {
            panic!("Expected a function");
        }
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
                .scope_manager
                .get_identifier(
                    node.value
                        .as_ref()
                        .expect("expected an identifier for value"),
                )
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
            Value::Function(_) => print!("function"),
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

    // fn get_top_level_identifiers(&self) -> HashMap<String, Value> {
    //     return self.identifiers.last().unwrap().clone();
    // }

    // fn insert_top_level_identifier(&mut self, identifier: String, value: Value) {
    //     let top = self.identifiers.last().unwrap();
    //     let mut new_top = top.clone();
    //     new_top.insert(identifier, value);
    //     self.identifiers.pop();
    //     self.identifiers.push(new_top);
    // }

    // fn get_function(&self, function_name: &str) -> Value {
    //     if !self.get_top_level_identifiers().contains_key(function_name) {
    //         panic!("Function not found");
    //     }

    //     return self
    //         .get_top_level_identifiers()
    //         .get(function_name)
    //         .unwrap()
    //         .clone();
    // }
}
