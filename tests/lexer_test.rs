use cloudy::cloudylang;

#[test]
fn number() {
    let input = "1";
    let expected = vec![cloudylang::utils::Token::new(
        cloudylang::utils::TokenKind::Number(1.0),
        cloudylang::utils::Position::new("<stdin>", input.to_string()),
    )];
    let actual = cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn float() {
    let input = "1.2";
    let expected = vec![cloudylang::utils::Token::new(
        cloudylang::utils::TokenKind::Number(1.2),
        cloudylang::utils::Position::new("<stdin>", input.to_string()),
    )];
    let actual = cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn plus_tok() {
    let input = "+";
    let expected = vec![cloudylang::utils::Token::new(
        cloudylang::utils::TokenKind::Plus,
        cloudylang::utils::Position::new("<stdin>", input.to_string()),
    )];
    let actual = cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn minus_tok() {
    let input = "-";
    let expected = vec![cloudylang::utils::Token::new(
        cloudylang::utils::TokenKind::Minus,
        cloudylang::utils::Position::new("<stdin>", input.to_string()),
    )];
    let actual = cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn mult_tok() {
    let input = "*";
    let expected = vec![cloudylang::utils::Token::new(
        cloudylang::utils::TokenKind::Mult,
        cloudylang::utils::Position::new("<stdin>", input.to_string()),
    )];
    let actual = cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn div_tok() {
    let input = "/";
    let expected = vec![cloudylang::utils::Token::new(
        cloudylang::utils::TokenKind::Div,
        cloudylang::utils::Position::new("<stdin>", input.to_string()),
    )];
    let actual = cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn lparen_tok() {
    let input = "(";
    let expected = vec![cloudylang::utils::Token::new(
        cloudylang::utils::TokenKind::LParen,
        cloudylang::utils::Position::new("<stdin>", input.to_string()),
    )];
    let actual = cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn rparen_tok() {
    let input = ")";
    let expected = vec![cloudylang::utils::Token::new(
        cloudylang::utils::TokenKind::RParen,
        cloudylang::utils::Position::new("<stdin>", input.to_string()),
    )];
    let actual = cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn invalid_char_error() {
    let input = "!";
    cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
}

#[test]
#[should_panic]
fn unexpected_char_error() {
    let input = "1.1.1";
    cloudylang::Lexer::new(input, "<stdin>").lex().unwrap();
}
