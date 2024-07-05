use crate::node::node::Node;

pub struct Interpreter {
    root: Node,
}

impl Interpreter {
    pub fn new(root: Node) -> Interpreter {
        Interpreter { root }
    }

    pub fn evaluate(&self) -> i32 {
        return self.evaluate_helper(&self.root);
    }

    fn evaluate_helper(&self, root: &Node) -> i32 {
        if root.children.len() == 0 {
            return root.value.parse::<i32>().unwrap();
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
