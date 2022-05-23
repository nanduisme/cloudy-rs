use super::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    NumberNode {
        tok: Token,
    },
    BinOpNode {
        left: Box<Node>,
        op: Token,
        right: Box<Node>,
    },
    UnaryOpNode {
        op: Token,
        right: Box<Node>,
    },
}
