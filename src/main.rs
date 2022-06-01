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
        } else if input.is_empty() {
            continue;
        }

        // Do all the parsing and interpreting
        match cloudylang::Interpreter::new().interpret(input, "<stdin>") {
            Ok(program) => {
                println!("{}", program.debug());
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}
