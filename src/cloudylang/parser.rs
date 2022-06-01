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
        self.tokens = Lexer::new(input, fname).lex()?;
        self.expr()
    }

    fn expr(&mut self) -> Result<Node, String> {
        self.rpn()
    }

    fn rpn(&mut self) -> Result<Node, String> {
        let left = self.unary()?;

        let mut stack: Vec<Token> = Vec::new();
        let mut output: Vec<Node> = vec![left];

        while self.current_tok().is_some()
            && !self.current_tok().unwrap().is_kind(&TokenKind::Eof)
            && self.current_tok().unwrap().of_kinds(&[
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Mult,
                TokenKind::Div,
                TokenKind::Mod,
                TokenKind::Pow,
            ])
        {
            let op = self.current_tok().unwrap().clone();
            self.advance();

            if !stack.is_empty() {
                match stack.get(0).unwrap().kind {
                    TokenKind::Plus | TokenKind::Minus => {
                        if op.of_kinds(&[TokenKind::Plus, TokenKind::Minus]) {
                            stack.reverse();
                            stack.push(op);
                            stack.reverse();

                            output.push(Node::Op {
                                op: stack.pop().unwrap(),
                            });
                        } else {
                            output.push(Node::Op { op })
                        }
                    }

                    TokenKind::Mult | TokenKind::Div => {
                        if op.of_kinds(&[
                            TokenKind::Mult,
                            TokenKind::Div,
                            TokenKind::Mod,
                            TokenKind::Plus,
                            TokenKind::Minus,
                        ]) {
                            stack.reverse();
                            stack.push(op);
                            stack.reverse();

                            output.push(Node::Op {
                                op: stack.pop().unwrap(),
                            });
                        } else {
                            output.push(Node::Op { op })
                        }
                    }
                    _ => {
                        stack.reverse();
                        stack.push(op);
                        stack.reverse();
                    }
                };
            } else {
                stack.push(op);
            }

            let right = match self.unary() {
                Ok(right) => right,
                Err(e) => return Err(e),
            };

            output.push(right);
        }

        while !stack.is_empty() {
            output.push(Node::Op {
                op: stack.pop().unwrap(),
            });
        }

        Ok(Node::Rpn { rpn: output })
    }

    fn unary(&mut self) -> Result<Node, String> {
        let tok = self.current_tok().unwrap().copy();
        if tok.of_kinds(&[TokenKind::Plus, TokenKind::Minus]) {
            self.advance();
            let unary = self.unary()?;
            return Ok(Node::UnaryOp {
                op: tok,
                right: Box::new(unary),
            });
        }

        self.atom()
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
            TokenKind::Number(_) => node = Node::Number { tok: tok.copy() },
            TokenKind::LParen => {
                self.advance();
                let expr = self.expr()?;

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
