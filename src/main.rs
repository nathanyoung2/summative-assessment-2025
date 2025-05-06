mod lexer;
use lexer::Lexer;

use std::io::{Write, stdin, stdout};

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
        let command_lexer = Lexer::new(&input);
        println!("{:?}", command_lexer.tokenize());
    }
}
