#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Expression,
    Equality,
    Term,
    Factor,
    Identifier,
    Operation,
    Literal,
}

#[derive(Clone)]
pub struct Node {
    pub value: Option<String>,
    pub node_type: NodeType,
    pub children: Vec<Node>,
}
