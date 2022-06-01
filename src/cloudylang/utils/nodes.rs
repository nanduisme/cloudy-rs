use super::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Number { tok: Token },
    UnaryOp { op: Token, right: Box<Node> },
    Op { op: Token },
    Rpn { rpn: Vec<Node> },
}
