#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Expression,
    Term,
    Factor,
    Identifier,
    Operation,
    Literal,
    If,
    Function,
    Args,
    Parameters,
    Program,
    Index,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub value: Option<String>,
    pub node_type: NodeType,
    pub children: Vec<Node>,
}
