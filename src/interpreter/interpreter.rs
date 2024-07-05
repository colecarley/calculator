use std::collections::HashMap;

use crate::node::node::Node;

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
        if root.value == "Assignment" {
            let identifier = &root.children[0].value;
            let value = self.evaluate_helper(&root.children[1]);
            self.identifiers.insert(identifier.clone(), value);
            return value;
        }

        if root.children.len() == 0 {
            let result = root.value.parse::<i32>();
            if result.is_ok() {
                return result.unwrap();
            } else {
                return *self.identifiers.get(&root.value).unwrap();
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

        match root.value.as_str() {
            "+" => values.iter().copied().reduce(|acc, el| acc + el).unwrap(),
            "-" => values.iter().copied().reduce(|acc, el| acc - el).unwrap(),
            "*" => values.iter().copied().reduce(|acc, el| acc * el).unwrap(),
            "/" => values.iter().copied().reduce(|acc, el| acc / el).unwrap(),
            _ => panic!("Invalid operator"),
        }
    }
}
