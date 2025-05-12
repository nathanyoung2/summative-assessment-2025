use crate::commands;

#[derive(Debug, Eq, PartialEq, Clone)]
/// Represents a token in the inputted command.
pub enum Token {
    Command(commands::CommandType),
    Word(String),
    PreviousDir,
    Space,
    Dot,
    Slash,
    And,
}

/// A `Lexer` is responsible for turning the raw input command from the user into a vector of
/// tokens that can be interpereted at a later stage.
pub struct Lexer<'s> {
    input: &'s str,
    cursor: usize,
}

impl<'s> Lexer<'s> {
    /// Create a new `Lexer`.
    /// Takes in the input source text
    pub fn new(input: &'s str) -> Self {
        Self { input, cursor: 0 }
    }
    
    /// Converts `self.input` into a vector of meaningful tokens.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut accumulator = Vec::new();

        while let Some(token) = self.read_next_token() {
            accumulator.push(token);
        }

        accumulator
    }

    /// Indentify the next token from the cursor
    fn read_next_token(&mut self) -> Option<Token> {
        if self.cursor == self.input.len() {
            return None;
        }

        // check for tokens with more than 1 character.
        if self.check_multi_token("&&") {
            return Some(Token::And);
        } else if self.check_multi_token("..") {
            return Some(Token::PreviousDir);
        } else if self.check_multi_token("touch") {
            return Some(Token::Command(commands::CommandType::Touch));
        } else if self.check_multi_token("cd") {
            return Some(Token::Command(commands::CommandType::Cd));
        } else if self.check_multi_token("mv") {
            return Some(Token::Command(commands::CommandType::Mv));
        } else if self.check_multi_token("ls") {
            return Some(Token::Command(commands::CommandType::Ls));
        }

        // check for tokens with 1 character.
        match self.input.chars().nth(self.cursor).unwrap() {
            '.' => {
                self.cursor += 1;
                Some(Token::Dot)
            }
            '/' => {
                self.cursor += 1;
                Some(Token::Slash)
            }
            ' ' => {
                self.cursor += 1;
                Some(Token::Space)
            }
            // any other token is treated as a `Word`
            // the start of the next token needs to be indentified to tell how long the word is.
            _ => {
                let next = self.next_token_index();
                let word_contents = &self.input[self.cursor..next];
                self.cursor = next;
                return Some(Token::Word(word_contents.to_string()));
            }
        }
    }

    /// Check if the cursor is currently at a multi-character long token: `token`
    fn check_multi_token(&mut self, token: &str) -> bool {
        if self.cursor + token.len() > self.input.len() {
            return false;
        }
        let chars = &self.input[self.cursor..(self.cursor + token.len())];
        if chars == token {
            self.cursor += token.len();
            return true;
        }

        false
    }

    /// Get the starting index of the next token that isn't a word.
    fn next_token_index(&self) -> usize {
        let chars = self.input[self.cursor..].chars();
        for (i, c) in chars.enumerate() {
            if matches![c, '.' | '/' | '&' | ' '] {
                return self.cursor + i;
            }
        }

        return self.input.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_with_file() {
        let input = "touch folder1/folder2/file.png";

        let expected_tokens = vec![
            Token::Command(commands::CommandType::Touch),
            Token::Space,
            Token::Word(String::from("folder1")),
            Token::Slash,
            Token::Word(String::from("folder2")),
            Token::Slash,
            Token::Word(String::from("file")),
            Token::Dot,
            Token::Word(String::from("png")),
        ];

        let mut command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn tokenize_with_trailing_folder() {
        let input = "cd folder1/folder2";

        let expected_tokens = vec![
            Token::Command(commands::CommandType::Cd),
            Token::Space,
            Token::Word(String::from("folder1")),
            Token::Slash,
            Token::Word(String::from("folder2")),
        ];

        let mut command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn tokenize_with_prev_directory() {
        let input = "cd ../folder1/folder2/file.png";

        let expected_tokens = vec![
            Token::Command(commands::CommandType::Cd),
            Token::Space,
            Token::PreviousDir,
            Token::Slash,
            Token::Word(String::from("folder1")),
            Token::Slash,
            Token::Word(String::from("folder2")),
            Token::Slash,
            Token::Word(String::from("file")),
            Token::Dot,
            Token::Word(String::from("png")),
        ];

        let mut command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn tokenize_multiple_commands() {
        let input = "cd folder1/folder2 && touch file.png";

        let expected_tokens = vec![
            Token::Command(commands::CommandType::Cd),
            Token::Space,
            Token::Word(String::from("folder1")),
            Token::Slash,
            Token::Word(String::from("folder2")),
            Token::Space,
            Token::And,
            Token::Space,
            Token::Command(commands::CommandType::Touch),
            Token::Space,
            Token::Word(String::from("file")),
            Token::Dot,
            Token::Word(String::from("png")),
        ];

        let mut command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn tokenize_two_arguments() {
        let input = "mv file1.png folder1/file1.png";

        let expected_tokens = vec![
            Token::Command(commands::CommandType::Mv),
            Token::Space,
            Token::Word(String::from("file1")),
            Token::Dot,
            Token::Word(String::from("png")),
            Token::Space,
            Token::Word(String::from("folder1")),
            Token::Slash,
            Token::Word(String::from("file1")),
            Token::Dot,
            Token::Word(String::from("png")),
        ];

        let mut command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }
}
