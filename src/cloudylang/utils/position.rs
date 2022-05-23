use std::fmt;

#[derive(Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub fname: &'static str,
    pub code: String,
    pub index: usize,
}

impl Position {
    pub fn new(fname: &'static str, code: String) -> Position {
        Position {
            line: 1,
            column: 1,
            fname,
            code,
            index: 0,
        }
    }

    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.index += 1;
    }

    pub fn copy(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
            fname: self.fname,
            code: self.code.clone(),
            index: self.index,
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Position {{ line: {}, column: {}, fname: {}, index: {} }}",
            self.line, self.column, self.fname, self.index
        )
    }
}
