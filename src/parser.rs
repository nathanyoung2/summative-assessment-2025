use crate::commands;
use crate::lexer::Token;

/// A `Parser` parses a vector of tokens into meaningful executable commands.
pub struct Parser {
    /// index of the current token being parsed
    cursor: usize,

    /// Current command builder that found arguments are being loaded into.
    current_command: Option<commands::CommandBuilder>,

    /// Previously parsed token.
    /// This is used for validating the token ordering.
    previous_token: Option<Token>,

    /// The starting index of the argument currently being parsed.
    arg_start: Option<usize>,

    /// The input tokens.
    tokens: Vec<Token>,
}

#[derive(Debug)]
/// A `Parser` will return this `SyntaxError` enum in the case that parsing fails.
pub enum SyntaxError {
    CommandNotProvided,
    InvalidCommand,
    InvalidPath,
    UnexpectedToken,
    InvalidArguments,
    InvalidType,
}

impl Parser {
    /// Create a new `Parser`.
    /// Takes in a vector of input `Token`s.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_command: None,
            previous_token: None,
            cursor: 0,
            arg_start: None,
        }
    }

    /// Generate a vector of executable commands.
    /// This can include 1 or more commands as commands can be chained with the && operator.
    pub fn generate_commands(&mut self) -> Result<Vec<Box<dyn commands::Command>>, SyntaxError> {
        if self.tokens.len() < 1 {
            return Err(SyntaxError::CommandNotProvided);
        }

        let mut commands = Vec::new();

        for token in self.tokens.iter() {
            // validate the order of the tokens for each token
            self.validate_token_order()?;

            match token {
                Token::Command(command_type) => {
                    // create a new command
                    self.current_command = Some(commands::CommandBuilder::new(command_type.clone()));
                },
                Token::And => {
                    // attempt to build the command
                    if let Some(command) = self.current_command.take() {
                        commands.push(command.build()?);
                    }
                },
                Token::Space => {
                    if let Some(arg_start) = self.arg_start {
                        // attempt to create an argument out of the accumulated tokens
                        let arg = compile_argument(&self.tokens[arg_start..self.cursor])?;

                        self.current_command.as_mut().map(|command| {
                            command.add_argument(arg);
                            command
                        });
                        self.arg_start = None;
                    }
                }
                Token::UnexpectedToken(token) => {
                    println!("Unexpected Token '{}'", token);
                }
                _ => {
                    // in the case where there are no tokens that perform
                    // operations themselves, set the current cursor position as the
                    // start of a new argument if there is no current argument being parsed.
                    if let None = self.arg_start {
                        self.arg_start = Some(self.cursor);
                    }
                }
            }

            // after the final token, compile an argument if there is one and 
            // attempt to build the command
            if self.cursor == self.tokens.len() - 1 {
                if let Some(arg_start) = self.arg_start {
                    let arg = compile_argument(&self.tokens[arg_start..])?;

                    self.current_command.as_mut().map(|command| {
                        command.add_argument(arg);
                        command
                    });
                    self.arg_start = None;
                }

                if let Some(command) = self.current_command.take() {
                    commands.push(command.build()?);
                }
            }

            self.previous_token = Some(token.clone());
            self.cursor += 1;
        }

        // return the accumulated commands
        return Ok(commands);
    }

    /// Validate the position of the current token in relation to the previous token.
    fn validate_token_order(&self) -> Result<(), SyntaxError> {
        match self.previous_token {
            Some(Token::Slash) => match self.tokens[self.cursor] {
                Token::Word(..) | Token::And | Token::Space => Ok(()),
                _ => Err(SyntaxError::UnexpectedToken),
            },
            Some(Token::PreviousDir) => {
                if let Token::Slash = self.tokens[self.cursor] {
                    Ok(())
                } else {
                    Err(SyntaxError::UnexpectedToken)
                }
            },
            Some(Token::Dot) => {
                if let Token::Word(..) = self.tokens[self.cursor] {
                    Ok(())
                } else {
                    Err(SyntaxError::UnexpectedToken)
                }
            },
            Some(Token::Word(..)) => match self.tokens[self.cursor] {
                Token::And | Token::Slash | Token::Dot | Token::Space => Ok(()),
                _ => Err(SyntaxError::UnexpectedToken),
            },
            Some(Token::Command(..)) => match self.tokens[self.cursor] {
                Token::Space => Ok(()),
                _ => Err(SyntaxError::UnexpectedToken),
            },
            Some(Token::Space) => match self.tokens[self.cursor] {
                Token::Dot => Err(SyntaxError::UnexpectedToken),
                _ => Ok(()),
            },
            Some(Token::And) => match self.tokens[self.cursor] {
                Token::Command(..) | Token::Space => Ok(()),
                _ => Err(SyntaxError::UnexpectedToken),
            },
            Some(Token::Number(..)) => match self.tokens[self.cursor] {
                Token::Space => Ok(()),
                _ => Err(SyntaxError::UnexpectedToken),
            }
            Some(Token::UnexpectedToken(..)) => Ok(()),
            None => {
                if let Token::Command(..) = self.tokens[self.cursor] {
                    Ok(())
                } else {
                    Err(SyntaxError::InvalidCommand)
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum Argument {
    Path(NodePath),
    Number(usize),
}

#[derive(Debug, Clone)]
/// Part of a NodePath
pub enum NodePathSegment {
    Root,
    Dir(String),
    Parent,
    File(String),
}

pub type NodePath = Vec<NodePathSegment>;

/// Helper function for converting an array of `Token`s into a `NodePath`.
/// Returns a `SyntaxError` if the path is not valid.
fn compile_argument(tokens: &[Token]) -> Result<Argument, SyntaxError> {
    match tokens.get(0) {
        Some(Token::Word { .. }) | Some(Token::Slash) 
        | Some(Token::PreviousDir) => {
            return compile_path(tokens).map(|path| Argument::Path(path));
        },
        Some(Token::Number(n)) => {
            return Ok(Argument::Number(*n));
        }
        _ => {
            return Err(SyntaxError::UnexpectedToken);
        }
    }
}

fn compile_path(tokens: &[Token]) -> Result<NodePath, SyntaxError> {
    let mut path = Vec::new();
    if let Some(Token::Slash) = tokens.get(0) {
        path.push(NodePathSegment::Root);
    }

    let mut tokens_iter = tokens.iter().peekable();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Word(name) => path.push(NodePathSegment::Dir(name.clone())),
            Token::PreviousDir => path.push(NodePathSegment::Parent),
            Token::Dot => {
                let last = path.last_mut().ok_or(SyntaxError::InvalidPath)?;
                let next = tokens_iter.peek().ok_or(SyntaxError::InvalidPath)?;

                if let Token::Word(extension_name) = *next {
                    match last { NodePathSegment::Dir(filename) => {
                            *last = NodePathSegment::File(format!("{}.{}", filename, extension_name));
                        },
                        _ => path.push(NodePathSegment::File(format!(".{}", extension_name))),
                    }
                    break;
                } else {
                    return Err(SyntaxError::InvalidPath);
                }
            }
            _ => continue,
        }
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_multi_command() {
        let input = "cd folder1/folder2 && touch file.png";
        let mut cmd_lexer = Lexer::new(input);
        let tokens = cmd_lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let commands = parser.generate_commands();

        assert!(commands.is_ok());
    }

    #[test]
    fn test_invalid_order() {
        let input = "cd .ab//as ls";
        let mut cmd_lexer = Lexer::new(input);
        let tokens = cmd_lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let commands = parser.generate_commands();

        assert!(!commands.is_ok());
    }

    #[test]
    fn test_valid_command() {
        let input = "cd abc/def/ghi";
        let mut cmd_lexer = Lexer::new(input);
        let tokens = cmd_lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let commands = parser.generate_commands();
        
        assert!(commands.is_ok());
    }

    #[test]
    fn test_cd_command_valid() {
        let input = "cd abc/def";
        let mut cmd_lexer = Lexer::new(input);
        let tokens = cmd_lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let commands = parser.generate_commands();

        assert!(commands.is_ok());
    }

    #[test]
    fn test_cd_command_invalid() {
        let input = "cd abc/def path2";
        let mut cmd_lexer = Lexer::new(input);
        let tokens = cmd_lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let commands = parser.generate_commands();

        assert!(!commands.is_ok());
    }
}
