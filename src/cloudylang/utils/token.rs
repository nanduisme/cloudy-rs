use super::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Value tokens
    Number(f64),

    // Single char tokens
    Plus,
    Minus,
    Mult,
    Div,
    LParen,
    RParen,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
}

impl Token {
    pub fn new(kind: TokenKind, pos: Position) -> Token {
        Token { kind, pos }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind
    }
}
