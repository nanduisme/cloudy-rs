use super::Position;

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
