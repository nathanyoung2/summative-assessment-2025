mod commands;

mod tree;
pub use tree::Context;

mod parser;
use parser::{Parser, SyntaxError};

mod lexer;
use lexer::Lexer;

use std::io::{Write, stdin, stdout};
use std::rc::Rc;

/// Helper function for reading a line of input.
fn get_user_input() -> Option<String> {
    stdout().flush().ok()?;

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).ok()?;

    Some(buffer)
}

/// Helper function to print error messages if a command cannot be built.
fn handle_err(e: SyntaxError) {
    match e {
        SyntaxError::CommandNotProvided => {
            println!("Please provide a command");
        },
        SyntaxError::InvalidCommand => {
            println!("The provided command is not valid");
        },
        SyntaxError::InvalidPath => {
            println!("The provided path is not valid");
        },
        SyntaxError::UnexpectedToken => {
            println!("Unexpected token in input");
        },
        SyntaxError::InvalidArguments => {
            println!("Arguments to the command are not valid");
        },
        SyntaxError::InvalidType => {
            println!("The type of an argument is not valid");
        }
    }
}

fn main() {
    // create the main context
    let ctx = Rc::new(tree::build_tree("user1"));

    loop {
        print!("{}> ", ctx.current_dir().borrow());
        let input = get_user_input();

        if let Some(input) = input {
            // create a lexer and tokenize the input string
            let mut command_lexer = Lexer::new(&input.trim());
            let tokens = command_lexer.tokenize();

            // pass the token array from the lexer to the parser to generate the commands
            let mut parser = Parser::new(tokens);
            let commands = parser.generate_commands();

            // execute the commands if they are valid.
            match commands {
                Ok(commands) => {
                    for command in commands.iter() {
                        command.execute(ctx.clone())
                    }
                },
                Err(e) => {
                    handle_err(e);
                }
            }
        }
    }
}
