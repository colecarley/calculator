#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Expression,
    Term,
    Factor,
    Identifier,
    Operation,
    Declaration,
    Assignment,
    Reassignment,
    FunctionCall,
    Literal,
    If,
    Function,
    Args,
    TypeAnnotation,
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
