use cloudy::cloudylang::{
    utils::{Node, Position, Token, TokenKind},
    Parser,
};

#[test]
fn number_parse() {
    let input = "1";
    let expected = Node::NumberNode {
        tok: Token::new(
            TokenKind::Number(1.0),
            Position::new("<stdin>", input.to_string()),
        ),
    };
    let actual = Parser::new().parse(input, "<stdin>").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn float_parse() {
    let input = "1.2";
    let expected = Node::NumberNode {
        tok: Token::new(
            TokenKind::Number(1.2),
            Position::new("<stdin>", input.to_string()),
        ),
    };
    let actual = Parser::new().parse(input, "<stdin>").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn simple_2_term_parse() {
    let input = "1 + 2";
    let expected = Node::BinOpNode {
        left: Box::new(Node::NumberNode {
            tok: Token::new(
                TokenKind::Number(1.0),
                Position::new("<stdin>", input.to_string()),
            ),
        }),
        op: Token::new(TokenKind::Plus, Position::new("<stdin>", input.to_string())),
        right: Box::new(Node::NumberNode {
            tok: Token::new(
                TokenKind::Number(2.0),
                Position::new("<stdin>", input.to_string()),
            ),
        }),
    };
    let actual = Parser::new().parse(input, "<stdin>").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn simple_multi_term_parse() {
    let input = "1 + 2 - 3 + 4 + 5 - 0";
    let expected = Node::BinOpNode {
        left: Box::new(Node::NumberNode {
            tok: Token::new(
                TokenKind::Number(1.0),
                Position::new("<stdin>", input.to_string()),
            ),
        }),
        op: Token::new(TokenKind::Plus, Position::new("<stdin>", input.to_string())),
        right: Box::new(Node::BinOpNode {
            left: Box::new(Node::NumberNode {
                tok: Token::new(
                    TokenKind::Number(2.0),
                    Position::new("<stdin>", input.to_string()),
                ),
            }),
            op: Token::new(TokenKind::Minus, Position::new("<stdin>", input.to_string())),
            right: Box::new(Node::BinOpNode {
                left: Box::new(Node::NumberNode {
                    tok: Token::new(
                        TokenKind::Number(3.0),
                        Position::new("<stdin>", input.to_string()),
                    ),
                }),
                op: Token::new(TokenKind::Plus, Position::new("<stdin>", input.to_string())),
                right: Box::new(Node::BinOpNode {
                    left: Box::new(Node::NumberNode {
                        tok: Token::new(
                            TokenKind::Number(4.0),
                            Position::new("<stdin>", input.to_string()),
                        ),
                    }),
                    op: Token::new(TokenKind::Plus, Position::new("<stdin>", input.to_string())),
                    right: Box::new(Node::BinOpNode {
                        left: Box::new(Node::NumberNode {
                            tok: Token::new(
                                TokenKind::Number(5.0),
                                Position::new("<stdin>", input.to_string()),
                            ),
                        }),
                        op: Token::new(
                            TokenKind::Minus,
                            Position::new("<stdin>", input.to_string()),
                        ),
                        right: Box::new(Node::NumberNode {
                            tok: Token::new(
                                TokenKind::Number(0.0),
                                Position::new("<stdin>", input.to_string()),
                            ),
                        }),
                    }),
                }),
            }),
        }),
    };

    let actual = Parser::new().parse(input, "<stdin>").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn simple_2_factor_parse() {
    let input = "1 * 2";
    let expected = Node::BinOpNode {
        left: Box::new(Node::NumberNode {
            tok: Token::new(
                TokenKind::Number(1.0),
                Position::new("<stdin>", input.to_string()),
            ),
        }),
        op: Token::new(TokenKind::Mult, Position::new("<stdin>", input.to_string())),
        right: Box::new(Node::NumberNode {
            tok: Token::new(
                TokenKind::Number(2.0),
                Position::new("<stdin>", input.to_string()),
            ),
        }),
    };
    let actual = Parser::new().parse(input, "<stdin>").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn simple_multi_factor_parse() {
    let input = "1 * 2 * 3 * 4 / 5 * 0";
    let expected = Node::BinOpNode {
        left: Box::new(Node::NumberNode {
            tok: Token::new(
                TokenKind::Number(1.0),
                Position::new("<stdin>", input.to_string()),
            ),
        }),
        op: Token::new(TokenKind::Mult, Position::new("<stdin>", input.to_string())),
        right: Box::new(Node::BinOpNode {
            left: Box::new(Node::NumberNode {
                tok: Token::new(
                    TokenKind::Number(2.0),
                    Position::new("<stdin>", input.to_string()),
                ),
            }),
            op: Token::new(TokenKind::Mult, Position::new("<stdin>", input.to_string())),
            right: Box::new(Node::BinOpNode {
                left: Box::new(Node::NumberNode {
                    tok: Token::new(
                        TokenKind::Number(3.0),
                        Position::new("<stdin>", input.to_string()),
                    ),
                }),
                op: Token::new(TokenKind::Mult, Position::new("<stdin>", input.to_string())),
                right: Box::new(Node::BinOpNode {
                    left: Box::new(Node::NumberNode {
                        tok: Token::new(
                            TokenKind::Number(4.0),
                            Position::new("<stdin>", input.to_string()),
                        ),
                    }),
                    op: Token::new(TokenKind::Div, Position::new("<stdin>", input.to_string())),
                    right: Box::new(Node::BinOpNode {
                        left: Box::new(Node::NumberNode {
                            tok: Token::new(
                                TokenKind::Number(5.0),
                                Position::new("<stdin>", input.to_string()),
                            ),
                        }),
                        op: Token::new(
                            TokenKind::Mult,
                            Position::new("<stdin>", input.to_string()),
                        ),
                        right: Box::new(Node::NumberNode {
                            tok: Token::new(
                                TokenKind::Number(0.0),
                                Position::new("<stdin>", input.to_string()),
                            ),
                        }),
                    }),
                }),
            }),
        }),
    };
    let actual = Parser::new().parse(input, "<stdin>").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn commutative_test_without_paren() {
    let input = "1 / 2 + 3";
    let expected = Node::BinOpNode {
        left: Box::new(Node::BinOpNode { 
            left: Box::new(Node::NumberNode {
                tok: Token::new(
                    TokenKind::Number(1.0),
                    Position::new("<stdin>", input.to_string()),
                ),
            }),
            op: Token::new(TokenKind::Div, Position::new("<stdin>", input.to_string())),
            right: Box::new(Node::NumberNode {
                tok: Token::new(
                    TokenKind::Number(2.0),
                    Position::new("<stdin>", input.to_string()),
                ),
            }),
        }),
        op: Token::new(TokenKind::Plus, Position::new("<stdin>", input.to_string())),
        right: Box::new(Node::NumberNode {
            tok: Token::new(
                TokenKind::Number(3.0),
                Position::new("<stdin>", input.to_string()),
            ),
        }),
    };

    let actual = Parser::new().parse(input, "<stdin>").unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn commutative_test_with_paren() {
    let input = "(1 + 1) * 2 + 3";
    let expected = Node::BinOpNode {
        left: Box::new(Node::BinOpNode {
            left: Box::new(Node::BinOpNode {
                left: Box::new(Node::NumberNode {
                    tok: Token::new(
                        TokenKind::Number(1.0),
                        Position::new("<stdin>", input.to_string()),
                    ),
                }),
                op: Token::new(TokenKind::Plus, Position::new("<stdin>", input.to_string())),
                right: Box::new(Node::NumberNode {
                    tok: Token::new(
                        TokenKind::Number(1.0),
                        Position::new("<stdin>", input.to_string()),
                    ),
                }),
            }),
            op: Token::new(TokenKind::Mult, Position::new("<stdin>", input.to_string())),
            right: Box::new(Node::NumberNode {
                tok: Token::new(
                    TokenKind::Number(2.0),
                    Position::new("<stdin>", input.to_string()),
                ),
            }),
        }),
        op: Token::new(TokenKind::Plus, Position::new("<stdin>", input.to_string())),
        right: Box::new(Node::NumberNode {
            tok: Token::new(
                TokenKind::Number(3.0),
                Position::new("<stdin>", input.to_string()),
            ),
        }),
    };
    let actual = Parser::new().parse(input, "<stdin>").unwrap();
    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn expected_number_after_plus() {
    let input = "1 +";
    Parser::new().parse(input, "<stdin>").unwrap();
}

#[test]
#[should_panic]
fn expected_number_after_minus() {
    let input = "1 -";
    Parser::new().parse(input, "<stdin>").unwrap();
}
