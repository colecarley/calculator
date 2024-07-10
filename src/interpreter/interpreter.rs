use core::panic;
use std::collections::HashMap;

use crate::node::node::{Node, NodeType};

pub struct Interpreter {
    identifiers: HashMap<String, i32>,
    functions: HashMap<String, Node>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            identifiers: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn new_with_identifiers(identifiers: HashMap<String, i32>) -> Interpreter {
        Interpreter {
            identifiers,
            functions: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, root: Node) -> i32 {
        return self.evaluate_helper(&root);
    }

    fn evaluate_helper(&mut self, root: &Node) -> i32 {
        if root.node_type == NodeType::Operation
        // && root.value.is_some()
        // && root.value.as_ref().unwrap() == "="
        {
            match &root.value {
                Some(val) => match val.clone().as_str() {
                    "=" => {
                        let identifier = root.children[0]
                            .value
                            .as_ref()
                            .expect("expected an identifier");
                        let value = self.evaluate_helper(&root.children[1]);
                        self.identifiers.insert(identifier.clone(), value);
                        return value;
                    }
                    "declaration" => {
                        let identifier = root.children[0]
                            .value
                            .as_ref()
                            .expect("expected an identifier");
                        self.functions
                            .insert(identifier.clone(), root.children[1].clone());
                        return 1;
                    }
                    _ => {
                        if self.functions.contains_key(val) {
                            let values: Vec<i32> = root.children[0]
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
            let result = root
                .value
                .as_ref()
                .expect("expected a value")
                .parse::<i32>();
            if result.is_ok() {
                return result.unwrap();
            } else {
                return *self
                    .identifiers
                    .get(
                        root.value
                            .as_ref()
                            .expect("expected an identifier for value"),
                    )
                    .unwrap();
            }
        }

        if root.children.len() == 1 {
            return self.evaluate_helper(&root.children[0]);
        }

        let values: Vec<i32> = root
            .children
            .iter()
            .map(|child| self.evaluate_helper(child))
            .collect();

        match root.value.as_ref().unwrap().as_str() {
            "+" => values.iter().copied().reduce(|acc, el| acc + el).unwrap(),
            "-" => values.iter().copied().reduce(|acc, el| acc - el).unwrap(),
            "*" => values.iter().copied().reduce(|acc, el| acc * el).unwrap(),
            "/" => values.iter().copied().reduce(|acc, el| acc / el).unwrap(),
            "==" => values
                .iter()
                .copied()
                .reduce(|acc, el| (acc == el) as i32)
                .unwrap(),
            "!=" => values
                .iter()
                .copied()
                .reduce(|acc, el| (acc != el) as i32)
                .unwrap(),
            ">" => values
                .iter()
                .copied()
                .reduce(|acc, el| (acc > el) as i32)
                .unwrap(),
            ">=" => values
                .iter()
                .copied()
                .reduce(|acc, el| (acc >= el) as i32)
                .unwrap(),
            "<" => values
                .iter()
                .copied()
                .reduce(|acc, el| (acc < el) as i32)
                .unwrap(),
            "<=" => values
                .iter()
                .copied()
                .reduce(|acc, el| (acc <= el) as i32)
                .unwrap(),
            "if" => {
                if values[0] != 0 {
                    return values[1];
                } else {
                    return values[2];
                }
            }
            _ => panic!("Invalid operator"),
        }
    }

    fn evaluate_function(&mut self, function_name: String, parameter_values: Vec<i32>) -> i32 {
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

        let mut arg_values: HashMap<String, i32> = HashMap::new();
        for (i, arg_name) in arg_names.iter().enumerate() {
            arg_values.insert(arg_name.clone(), parameter_values[i]);
        }

        let mut interpreter = Interpreter::new_with_identifiers(arg_values);

        let result = interpreter.evaluate(function.children[1].clone());

        result
    }
}
