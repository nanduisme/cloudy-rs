use super::{Position, Token};

pub struct Errors;
impl Errors {
    pub fn invalid_char_error(c: char, pos: Position) -> String {
        Errors::error_shell(
            "InvalidCharError",
            pos,
            format!("Invalid character '{}'", c),
        )
    }

    pub fn unexpected_char_error(c: char, pos: Position) -> String {
        Errors::error_shell(
            "UnexpectedCharError",
            pos,
            format!("Unexpected character '{}'", c),
        )
    }

    pub fn unexpected_eof_error(pos: Position) -> String {
        Errors::error_shell(
            "UnexpectedEofError",
            pos,
            "Unexpected end of file".to_string(),
        )
    }

    pub fn unexpected_token_error(tok: Token, expected: &str) -> String {
        Errors::error_shell(
            "UnexpectedTokenError",
            tok.pos.copy(),
            format!("Unexpected token '{}' (expected {})", tok.name(), expected),
        )
    }

    pub fn type_error(pos: Position, expected: &'static str, found: &str) -> String {
        Errors::error_shell(
            "TypeError",
            pos,
            format!("Expected {} but found {}", expected, found),
        )
    }

    pub fn div_by_zero_error(pos: Position) -> String {
        Errors::error_shell(
            "DivByZeroError",
            pos,
            "Division by zero".to_string(),
        )
    }

    fn error_shell(err_name: &'static str, pos: Position, message: String) -> String {
        format!(
            "{} at {} {}:{} \n\t {} \n\t {}^\n{}",
            err_name,
            pos.fname,
            pos.line,
            pos.column,
            pos.code,
            " ".repeat(pos.column - 1),
            message
        )
    }
}
