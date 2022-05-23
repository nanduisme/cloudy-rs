use super::{
    utils::{Errors, Node, Token, TokenKind},
    Lexer,
};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            index: 0,
        }
    }

    fn current_tok(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    pub fn parse(&mut self, input: &str, fname: &'static str) -> Result<Node, String> {
        self.tokens = match Lexer::new(input, fname).lex() {
            Ok(tokens) => tokens,
            Err(e) => return Err(e),
        };

        self.expr()
    }

    fn expr(&mut self) -> Result<Node, String> {
        self.term()
    }

    fn term(&mut self) -> Result<Node, String> {
        let factor = match self.factor() {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let tok = self.current_tok().unwrap().copy();
        if tok.of_kinds(&[TokenKind::Plus, TokenKind::Minus]) {
            self.advance();
            let term = match self.term() {
                Ok(t) => t,
                Err(e) => return Err(e),
            };
            return Ok(Node::BinOpNode {
                left: Box::new(factor),
                op: tok,
                right: Box::new(term),
            });
        }
        Ok(factor)
    }

    fn factor(&mut self) -> Result<Node, String> {
        let atom = match self.atom() {
            Ok(atom) => atom,
            Err(e) => return Err(e),
        };

        let tok = self.current_tok().unwrap().copy();
        if tok.of_kinds(&[TokenKind::Mult, TokenKind::Div]) {
            self.advance();
            let right = match self.factor() {
                Ok(right) => right,
                Err(e) => return Err(e),
            };

            return Ok(Node::BinOpNode {
                left: Box::new(atom),
                op: tok,
                right: Box::new(right),
            });
        }

        Ok(atom)
    }

    fn atom(&mut self) -> Result<Node, String> {
        if self.current_tok().is_none() || self.current_tok().unwrap().is_kind(&TokenKind::Eof) {
            return Err(Errors::unexpected_eof_error(
                self.current_tok().unwrap().pos.copy(),
            ));
        }

        let tok = self.current_tok().unwrap();
        let node: Node;

        match tok.kind {
            TokenKind::Number(_) => node = Node::NumberNode { tok: tok.copy() },
            TokenKind::LParen => {
                self.advance();

                let expr = match self.expr() {
                    Ok(expr) => expr,
                    Err(e) => return Err(e),
                };

                if self.current_tok().unwrap().is_kind(&TokenKind::RParen) {
                    node = expr;
                } else {
                    return Err(Errors::unexpected_token_error(
                        self.current_tok().unwrap().copy(),
                        "')'",
                    ));
                }
            }
            _ => return Err(Errors::unexpected_token_error(tok.copy(), "number or '('")),
        }

        self.advance();
        Ok(node)
    }
}
