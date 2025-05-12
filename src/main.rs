mod commands;

mod tree;
pub use tree::Context;

mod parser;
use parser::Parser;

mod lexer;
use lexer::Lexer;

use std::io::{Write, stdin, stdout};

/// Helper function for reading a line of input.
fn get_user_input() -> Option<String> {
    stdout().flush().ok()?;

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).ok()?;

    Some(buffer)
}

fn main() {
    print!("/home/myuser/> ");
    let input = get_user_input();

    if let Some(input) = input {
        let mut command_lexer = Lexer::new(&input);
        let tokens = command_lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let commands = parser.generate_commands();
    }
}
