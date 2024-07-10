use core::panic;
use std::collections::HashMap;

use crate::node::node::{Node, NodeType};

pub struct Interpreter {
    identifiers: HashMap<String, i32>,
    functions: HashMap<String, Node>,
    lists: HashMap<String, Vec<i32>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            identifiers: HashMap::new(),
            functions: HashMap::new(),
            lists: HashMap::new(),
        }
    }

    fn new_with_starting_values(
        identifiers: HashMap<String, i32>,
        functions: HashMap<String, Node>,
        lists: HashMap<String, Vec<i32>>,
    ) -> Interpreter {
        Interpreter {
            identifiers,
            functions,
            lists,
        }
    }

    pub fn evaluate(&mut self, root: Node) -> i32 {
        return self.evaluate_helper(&root);
    }

    fn evaluate_helper(&mut self, root: &Node) -> i32 {
        if root.node_type == NodeType::Operation {
            match &root.value {
                Some(val) => match val.clone().as_str() {
                    "=" => {
                        let identifier = root.children[0]
                            .value
                            .as_ref()
                            .expect("expected an identifier");
                        if root.children[1].node_type == NodeType::List {
                            let values: Vec<i32> = root.children[1]
                                .children
                                .iter()
                                .map(|child| self.evaluate_helper(child))
                                .collect();
                            self.lists.insert(identifier.clone(), values);
                            return 1;
                        } else {
                            let value = self.evaluate_helper(&root.children[1]);
                            self.identifiers.insert(identifier.clone(), value);
                            return value;
                        }
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
                    "head" => {
                        let identifier: &String =
                            root.children[0].children[0].children[0].children[0].children[0]
                                .value
                                .as_ref()
                                .expect("expected an identifier");

                        let list = self.lists.get(identifier).unwrap();
                        return list[0];
                    }
                    "tail" => {
                        // let identifier: &String =
                        //     root.children[0].children[0].children[0].children[0].children[0]
                        //         .value
                        //         .as_ref()
                        //         .expect("expected an identifier");
                        // let list = self.lists.get(identifier).unwrap();
                        // let mut new_list = list.clone();
                        // new_list.remove(0);
                        // self.lists.insert(identifier.clone(), new_list);
                        // return 1;
                    }
                    "len" => {
                        let identifier: &String =
                            root.children[0].children[0].children[0].children[0].children[0]
                                .value
                                .as_ref()
                                .expect("expected an identifier");

                        let list = self.lists.get(identifier).unwrap();
                        return list.len() as i32;
                    }
                    "print" => {
                        let value = self.evaluate_helper(&root.children[0]);
                        println!("{:?}", value);
                        return value;
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
                let result = self.identifiers.get(
                    root.value
                        .as_ref()
                        .expect("expected an identifier for value"),
                );
                if result.is_some() {
                    return *result.unwrap();
                } else {
                    let result = self.lists.get(
                        root.value
                            .as_ref()
                            .expect("expected an identifier for value"),
                    );
                    println!("{:?}", result.unwrap());
                    if result.is_some() {
                        return 1;
                    }
                }
            }
        }

        if root.children.len() == 1 {
            return self.evaluate_helper(&root.children[0]);
        }

        if root.node_type == NodeType::ArrayIndex {
            let list_name = root.children[0]
                .value
                .as_ref()
                .expect("expected an identifier for value");
            let index = self.evaluate_helper(&root.children[1]);

            let list = self.lists.get(list_name).unwrap();
            return list[index as usize];
        }

        if root.node_type == NodeType::If {
            let condition = self.evaluate_helper(&root.children[0]);
            if condition != 0 {
                return self.evaluate_helper(&root.children[1]);
            } else {
                return self.evaluate_helper(&root.children[2]);
            }
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
            "%" => values.iter().copied().reduce(|acc, el| acc % el).unwrap(),
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

        let mut interpreter = Interpreter::new_with_starting_values(
            arg_values,
            self.functions.clone(),
            self.lists.clone(),
        );

        let result = interpreter.evaluate(function.children[1].clone());

        result
    }
}
