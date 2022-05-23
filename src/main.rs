use std::io::{self, Write};
mod cloudylang;

fn main() {
    // Basic Shell

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        // Do all the parsing and interpreting
        match cloudylang::Parser::new().parse(input, "<stdin>") {
            Ok(program) => {
                println!("{:?}", program);
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}
