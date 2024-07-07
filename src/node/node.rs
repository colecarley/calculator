#[derive(Debug, PartialEq)]
pub enum NodeType {
    Expression,
    Term,
    Factor,
    Identifier,
    Literal,
}

pub struct Node {
    pub value: Option<String>,
    pub node_type: NodeType,
    pub children: Vec<Node>,
}
