#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Expression,
    Term,
    Factor,
    Identifier,
    Operation,
    Declaration,
    Assignment,
    FunctionCall,
    Literal,
    If,
    Function,
    Args,
    Parameters,
    Program,
    Index,
    List,
    Block,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub value: Option<String>,
    pub node_type: NodeType,
    pub children: Vec<Node>,
}
