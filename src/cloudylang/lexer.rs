use super::utils::{Errors, Position, Token, TokenKind};

pub struct Lexer {
    input: String,
    pos: Position,
    numbers: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str, fname: &'static str) -> Self {
        Lexer {
            pos: Position::new(fname, <&str>::clone(&input).to_string()),
            input: input.to_string(),
            numbers: "0123456789.".chars().collect::<Vec<char>>(),
        }
    }

    fn advance(&mut self) {
        self.pos.advance(self.current_char().unwrap());
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let code = &self.input.chars().collect::<Vec<char>>();
        let mut tokens: Vec<Token> = Vec::new();

        while self.current_char().is_some() && self.pos.index <= code.len() {
            match &self.current_char().unwrap() {
                ' ' => self.advance(),
                x if self.numbers.contains(x) => match self.make_num() {
                    Ok(num) => tokens.push(num),
                    Err(e) => return Err(e),
                },
                '+' => self.push_sing_char_tok(&mut tokens, TokenKind::Plus, self.pos.copy()),
                '-' => self.push_sing_char_tok(&mut tokens, TokenKind::Minus, self.pos.copy()),
                '*' => self.push_sing_char_tok(&mut tokens, TokenKind::Mult, self.pos.copy()),
                '/' => self.push_sing_char_tok(&mut tokens, TokenKind::Div, self.pos.copy()),
                '(' => self.push_sing_char_tok(&mut tokens, TokenKind::LParen, self.pos.copy()),
                ')' => self.push_sing_char_tok(&mut tokens, TokenKind::RParen, self.pos.copy()),
                _ => {
                    return Err(Errors::invalid_char_error(
                        self.current_char().unwrap(),
                        self.pos.copy(),
                    ))
                }
            }
        }

        tokens.push(Token::new(TokenKind::Eof, self.pos.copy()));
        Ok(tokens)
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos.index)
    }

    fn make_num(&mut self) -> Result<Token, String> {
        let mut num = String::new();
        let pos = self.pos.copy();
        let mut found_decimal = false;

        while self.current_char().is_some() && self.numbers.contains(&self.current_char().unwrap())
        {
            if self.current_char().unwrap() == '.' && found_decimal {
                return Err(Errors::unexpected_char_error('.', self.pos.copy()));
            } else if self.current_char().unwrap() == '.' {
                found_decimal = true;
            }

            num.push(self.current_char().unwrap());
            self.advance();
        }

        Ok(Token::new(TokenKind::Number(num.parse::<f64>().unwrap()), pos))
    }

    fn push_sing_char_tok(&mut self, toks: &mut Vec<Token>, kind: TokenKind, pos: Position) {
        toks.push(Token::new(kind, pos));
        self.advance();
    }
}
