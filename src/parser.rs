use crate::commands;
use crate::lexer::Token;

pub struct Parser {
    cursor: usize,

    contains_required_arguments: bool,

    current_command: Option<Box<dyn commands::Command>>,
    previous_token: Option<Token>,

    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum SyntaxError {
    CommandNotProvided,
    InvalidCommand,
    UnexpectedToken,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            contains_required_arguments: false,
            current_command: None,
            previous_token: None,
            cursor: 0,
        }
    }

    pub fn generate_commands(&mut self) -> Result<Vec<Box<dyn commands::Command>>, SyntaxError> {
        if self.tokens.len() < 1 {
            return Err(SyntaxError::CommandNotProvided);
        }

        for token in self.tokens.iter() {
            self.validate_token_order()?;
            self.previous_token = Some(self.tokens[self.cursor].clone());
            self.cursor += 1
        }

        let commands = Vec::new();

        return Ok(commands);
    }

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
            }
            Some(Token::Dot) => {
                if let Token::Word(..) = self.tokens[self.cursor] {
                    Ok(())
                } else {
                    Err(SyntaxError::UnexpectedToken)
                }
            }
            Some(Token::Word(..)) => match self.tokens[self.cursor] {
                Token::And | Token::Slash | Token::Dot => Ok(()),
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
            None => {
                if let Token::Command(..) = self.tokens[self.cursor] {
                    Ok(())
                } else {
                    Err(SyntaxError::InvalidCommand)
                }
            }
        }
    }
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
}
