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
}

#[derive(Debug, Clone)]
pub struct Node {
    pub value: Option<String>,
    pub node_type: NodeType,
    pub children: Vec<Node>,
}
