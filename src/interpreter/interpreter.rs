use std::collections::HashMap;

use crate::node::node::{Node, NodeType};

pub struct Interpreter {
    identifiers: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            identifiers: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, root: Node) -> i32 {
        return self.evaluate_helper(&root);
    }

    fn evaluate_helper(&mut self, root: &Node) -> i32 {
        if root.node_type == NodeType::Expression
            && root.value.is_some()
            && root.value.as_ref().unwrap() == "="
        {
            let identifier = root.children[0]
                .value
                .as_ref()
                .expect("expected an identifier");
            let value = self.evaluate_helper(&root.children[1]);
            self.identifiers.insert(identifier.clone(), value);
            return value;
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
                    .get(root.value.as_ref().expect("expected an identifier"))
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
}
