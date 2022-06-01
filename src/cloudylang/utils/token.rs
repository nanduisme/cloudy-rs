use super::Position;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenKind {
    // Value tokens
    Number(f64),

    Eof,

    // Operators
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Pow,
    
    // Single char tokens
    LParen,
    RParen,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
}

impl Token {
    pub fn new(kind: TokenKind, pos: Position) -> Token {
        Token { kind, pos }
    }

    pub fn copy(&self) -> Token {
        Token {
            kind: self.kind,
            pos: self.pos.copy(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self.kind {
            TokenKind::Number(_) => "number",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Mult => "*",
            TokenKind::Div => "/",
            TokenKind::Mod => "%",
            TokenKind::Pow => "^",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::Eof => "eof",
        }
    }

    pub fn is_kind(&self, other: &TokenKind) -> bool {
        self.kind == *other
    }

    pub fn of_kinds(&self, kinds: &[TokenKind]) -> bool {
        kinds.iter().any(|k| self.is_kind(k))
    }

}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tok {{ kind: {:?} }}", self.kind)
    }
}
