use core::panic;
use std::collections::HashMap;

use crate::{
    node::node::{Node, NodeType},
    token::token::Value,
};

struct ScopeManager {
    scopes: Vec<HashMap<String, Value>>,
    num: i32,
}

impl ScopeManager {
    fn new() -> ScopeManager {
        let scopes = vec![HashMap::new()];
        ScopeManager { scopes, num: 1 }
    }

    fn insert_identifier(&mut self, identifier: String, value: Value) {
        let top = self.scopes.get_mut((self.num - 1) as usize).unwrap();
        top.insert(identifier, value);
    }

    fn reassign_identifier(&mut self, identifier: String, value: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&identifier) {
                scope.insert(identifier, value);
                return;
            }
        }
        panic!("Identifier not found");
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
        self.scopes.push(HashMap::new());
        self.num += 1;
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
        self.num -= 1;
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
        self.store_functions(&root);

        let mut result = Value::Number(0);
        let mut early_return = false;
        for child in &root.children {
            result = self.evaluate_helper(child, &mut early_return);
            if early_return {
                break;
            }
        }
        result
    }

    fn evaluate_helper(&mut self, root: &Node, early_return: &mut bool) -> Value {
        match root.node_type {
            NodeType::FunctionCall => {
                let val = root.value.as_ref().unwrap().as_str();
                if self.scope_manager.contains_identifier(val) {
                    return self.handle_function_call(root, val);
                } else {
                    match val {
                        "print" => return self.handle_print(root),
                        "println" => return self.handle_println(root),
                        "head" => return self.handle_head(root),
                        "tail" => return self.handle_tail(root),
                        "len" => return self.handle_len(root),
                        "type" => return self.handle_type(root),
                        "is_bool" => return self.handle_is_bool(root),
                        "is_number" => return self.handle_is_number(root),
                        "is_string" => return self.handle_is_string(root),
                        "is_list" => return self.handle_is_list(root),
                        "is_function" => return self.handle_is_function(root), //TODO: fix this

                        "input" => return self.handle_input(),
                        _ => {
                            panic!("Function not found");
                        }
                    }
                }
            }
            NodeType::Assignment => {
                return self.handle_assignment(root);
            }
            NodeType::Reassignment => {
                return self.handle_reassignment(root);
            }
            NodeType::Declaration => {
                return self.handle_declaration(root);
            }
            NodeType::Block => {
                return self.handle_block(root, early_return);
            }
            NodeType::List => {
                return self.handle_list(root);
            }
            NodeType::If => {
                return self.handle_if(root, early_return);
            }
            NodeType::Index => {
                return self.handle_index(root);
            }
            NodeType::Operation => {
                return self.handle_operator(root);
            }
            NodeType::Identifier | NodeType::Literal => {
                return self.parse_value(root);
            }
            NodeType::Factor | NodeType::Term | NodeType::Expression | NodeType::Args => {
                // just wrapper nodes
                if root.children.len() != 1 {
                    panic!("Invalid number of children for {:?}", root.node_type);
                }
                return self.evaluate_helper(&root.children[0], early_return);
            }
            NodeType::Return => {
                *early_return = true;
                return self.evaluate_helper(&root.children[0], early_return);
            }
            NodeType::Parameters => {
                // just wrapper nodes
                panic!("Parameters node should not be evaluated");
            }
            NodeType::Program => {
                // just wrapper nodes
                panic!("Program node should not be evaluated");
            }

            _ => {
                panic!("Invalid node type {:?}", root.node_type);
            }
        }
    }

    fn evaluate_function(&mut self, function: &Value, parameter_values: Vec<Value>) -> Value {
        if let Value::Function(function) = function {
            let param_names: Vec<String> = function.children[0]
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
            for (i, param_name) in param_names.iter().enumerate() {
                arg_values.insert(param_name.clone(), parameter_values[i].clone());
            }

            self.scope_manager.new_scope_with_values(arg_values);
            let mut early_return = false;
            for child in function.children[1]
                .children
                .iter()
                .take(function.children[1].children.len() - 1)
            {
                let result = self.evaluate_helper(child, &mut early_return);
                if early_return {
                    self.scope_manager.pop_scope();
                    return result;
                }
            }

            let result = self.evaluate_helper(
                function.children[1]
                    .children
                    .last()
                    .expect("expected a child"),
                &mut false,
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
            panic!("Invalid value {:?}", node.node_type);
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

    fn handle_print(&mut self, root: &Node) -> Value {
        let value = self.evaluate_helper(&root.children[0], &mut false);
        self.print_value(&value);
        return value;
    }

    fn handle_println(&mut self, root: &Node) -> Value {
        let value = self.evaluate_helper(&root.children[0], &mut false);
        self.print_value(&value);
        println!();
        return value;
    }

    fn handle_head(&mut self, root: &Node) -> Value {
        let list = self.evaluate_helper(&root.children[0], &mut false);
        if let Value::List(list) = list {
            return list[0].clone();
        } else {
            panic!("Expected a list");
        }
    }

    fn handle_tail(&mut self, root: &Node) -> Value {
        let list = self.evaluate_helper(&root.children[0], &mut false);
        if let Value::List(list) = list {
            return Value::List(list[1..].to_vec().clone());
        } else if let Value::String(string) = list {
            return Value::String(string[1..].to_string());
        } else {
            panic!("Expected a list or a string");
        }
    }

    fn handle_len(&mut self, root: &Node) -> Value {
        let list = self.evaluate_helper(&root.children[0], &mut false);
        if let Value::List(list) = list {
            return Value::Number(list.len() as i32);
        } else if let Value::String(string) = list {
            return Value::Number(string.len() as i32);
        } else {
            panic!("Expected a list or a string");
        }
    }

    fn handle_type(&mut self, root: &Node) -> Value {
        let value = self.evaluate_helper(&root.children[0], &mut false);
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

    fn handle_is_bool(&mut self, root: &Node) -> Value {
        let value = self.evaluate_helper(&root.children[0], &mut false);
        return match value {
            Value::Boolean(_) => Value::Boolean(true),
            _ => Value::Boolean(false),
        };
    }

    fn handle_is_number(&mut self, root: &Node) -> Value {
        let value = self.evaluate_helper(&root.children[0], &mut false);
        return match value {
            Value::Number(_) => Value::Boolean(true),
            _ => Value::Boolean(false),
        };
    }

    fn handle_is_string(&mut self, root: &Node) -> Value {
        let value = self.evaluate_helper(&root.children[0], &mut false);
        return match value {
            Value::String(_) => Value::Boolean(true),
            _ => Value::Boolean(false),
        };
    }

    fn handle_is_list(&mut self, root: &Node) -> Value {
        let value = self.evaluate_helper(&root.children[0], &mut false);
        return match value {
            Value::List(_) => Value::Boolean(true),
            _ => Value::Boolean(false),
        };
    }

    fn handle_is_function(&mut self, root: &Node) -> Value {
        let args: Vec<Value> = root
            .children
            .iter()
            .map(|child| self.evaluate_helper(child, &mut false))
            .collect();
        let val = &args[0];
        if let Value::Function(_) = val {
            Value::Boolean(true)
        } else {
            Value::Boolean(false)
        }
    }

    fn handle_input(&mut self) -> Value {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        return Value::String(input.trim().to_string());
    }

    fn handle_assignment(&mut self, root: &Node) -> Value {
        let identifier = root.children[0]
            .value
            .as_ref()
            .expect("expected an identifier");
        let value = self.evaluate_helper(&root.children[1], &mut false);
        self.scope_manager
            .insert_identifier(identifier.clone(), value.clone());
        return value;
    }

    fn handle_reassignment(&mut self, root: &Node) -> Value {
        let identifier = root.children[0]
            .value
            .as_ref()
            .expect("expected an identifier");
        let value = self.evaluate_helper(&root.children[1], &mut false);
        self.scope_manager
            .reassign_identifier(identifier.clone(), value.clone());
        return value;
    }

    fn handle_declaration(&mut self, root: &Node) -> Value {
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

    fn handle_block(&mut self, root: &Node, early_return: &mut bool) -> Value {
        for child in root.children.iter().take(root.children.len() - 1) {
            let result = self.evaluate_helper(child, early_return);
            if *early_return {
                return result;
            }
        }
        return self.evaluate_helper(
            root.children.last().expect("expected a child"),
            early_return,
        );
    }

    fn handle_list(&mut self, root: &Node) -> Value {
        let values: Vec<Value> = root
            .children
            .iter()
            .map(|child| self.evaluate_helper(child, &mut false))
            .collect();

        return Value::List(values);
    }

    fn handle_index(&mut self, root: &Node) -> Value {
        let indexable = self.evaluate_helper(&root.children[0], &mut false);

        let index = self.evaluate_helper(&root.children[1], &mut false);
        let index = if let Value::Number(index) = index {
            index
        } else {
            panic!("Expected a number");
        };

        if let Value::String(string) = indexable {
            return Value::String((string.as_bytes()[index as usize].clone() as char).to_string());
        }
        if let Value::List(list) = indexable {
            return list[index as usize].clone();
        } else {
            panic!("Expected a string or a list");
        }
    }

    fn handle_if(&mut self, root: &Node, early_return: &mut bool) -> Value {
        let condition = self.evaluate_helper(&root.children[0], &mut false);
        let condition = match condition {
            Value::Boolean(val) => val,
            _ => panic!("Expected a boolean"),
        };

        if condition {
            if root.children.len() >= 2 {
                let result = self.evaluate_helper(&root.children[1], early_return);
                return result;
            } else {
                return Value::Null;
            }
        } else {
            if root.children.len() >= 3 {
                return self.evaluate_helper(&root.children[2], &mut false);
            } else {
                return Value::Null;
            }
        }
    }

    fn handle_operator(&mut self, root: &Node) -> Value {
        let values: Vec<Value> = root
            .children
            .iter()
            .map(|child| self.evaluate_helper(child, &mut false))
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
                    if values.len() == 1 {
                        return Value::Number(-first);
                    }
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

    fn handle_function_call(&mut self, root: &Node, val: &str) -> Value {
        let values: Vec<Value> = root.children[0]
            .children
            .iter()
            .map(|child| self.evaluate_helper(child, &mut false))
            .collect();

        let function = self.scope_manager.get_identifier(val).clone();
        return self.evaluate_function(&function, values);
    }

    fn store_functions(&mut self, root: &Node) -> Value {
        for child in &root.children {
            if child.node_type == NodeType::Declaration {
                let identifier = child.children[0]
                    .value
                    .as_ref()
                    .expect("expected an identifier");
                self.scope_manager.insert_identifier(
                    identifier.clone(),
                    Value::Function(child.children[1].clone()),
                );
            } else {
                self.store_functions(child);
            }
        }
        return Value::Null;
    }
}
