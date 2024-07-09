#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Expression,
    Term,
    Factor,
    Identifier,
    Operation,
    Literal,
    If,
}

#[derive(Clone)]
pub struct Node {
    pub value: Option<String>,
    pub node_type: NodeType,
    pub children: Vec<Node>,
}
