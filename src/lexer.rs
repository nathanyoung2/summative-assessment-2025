#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Command { value: String },
    Folder { value: String },
    File { value: String },
    PreviousDir,
    Dot,
    Slash,
    Extension { value: String },
}

/// A `Lexer` is responsible for turning the raw input command from the user into a vector of
/// tokens that can be interpereted at a later stage.
pub struct Lexer {
    input: String,
}

impl Lexer {
    /// Create a new `Lexer`.
    /// Takes in the input command
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    /// Tokenize the input command and return it as a `Vec<Token>`
    pub fn tokenize(&self) -> Vec<Token> {
        let mut accumulator = Vec::new();
        // iterate over every command, multiple commands can be combined with `&&`
        for line in self.input.split("&&") {
            let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();

            // Tokenize the command
            if let Some(command) = parts.get(0) {
                accumulator.push(Token::Command {
                    value: command.to_string(),
                });
            }

            // Tokenize the path argument to the command
            if let Some(path) = parts.get(1) {
                Self::tokenize_path(&mut accumulator, path);
            }
        }

        accumulator
    }

    /// Helper function to tokenize a directory path.
    /// Takes in an accumulator vector that the resulting tokens will be pushed to as well as the
    /// path string itself.
    fn tokenize_path(accumulator: &mut Vec<Token>, path: &str) {
        // current word index (start of the current word)
        let mut cwi = 0;

        // iterate over all characters
        for (i, c) in path.chars().enumerate() {
            // if a slash is found, add the folder before the slash
            // and the slash itself as tokens
            if c == '/' {
                let prev_word = path[cwi..i].to_string();
                accumulator.push(match prev_word.as_str() {
                    ".." => Token::PreviousDir,
                    _ => Token::Folder { value: prev_word },
                });
                accumulator.push(Token::Slash);
                cwi = i + 1;
            }
        }

        // tokenise file and extension
        let last = path.split("/").last();
        if let Some(last) = last {
            // if there is no `.` found in the last segment of the path it
            // means that the path ends with a folder
            if !last.contains(".") {
                accumulator.push(Token::Folder {
                    value: last.to_string(),
                })
            }

            // otherwise add the file, dot, and extension as a token
            for (i, c) in last.chars().enumerate() {
                if c == '.' {
                    accumulator.push(Token::File {
                        value: last[0..i].to_string(),
                    });
                    accumulator.push(Token::Dot);

                    if let Some(_) = last.chars().collect::<Vec<char>>().get(i + 1) {
                        accumulator.push(Token::Extension {
                            value: last[i + 1..].to_string(),
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_with_file() {
        let input = "touch folder1/folder2/file.png";

        let expected_tokens = vec![
            Token::Command {
                value: String::from("touch"),
            },
            Token::Folder {
                value: String::from("folder1"),
            },
            Token::Slash,
            Token::Folder {
                value: String::from("folder2"),
            },
            Token::Slash,
            Token::File {
                value: String::from("file"),
            },
            Token::Dot,
            Token::Extension {
                value: String::from("png"),
            },
        ];

        let command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn tokenize_with_trailing_folder() {
        let input = "cd folder1/folder2";

        let expected_tokens = vec![
            Token::Command {
                value: String::from("cd"),
            },
            Token::Folder {
                value: String::from("folder1"),
            },
            Token::Slash,
            Token::Folder {
                value: String::from("folder2"),
            },
        ];

        let command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn tokenize_with_prev_directory() {
        let input = "cd ../folder1/folder2/file.png";

        let expected_tokens = vec![
            Token::Command {
                value: String::from("cd"),
            },
            Token::PreviousDir,
            Token::Slash,
            Token::Folder {
                value: String::from("folder1"),
            },
            Token::Slash,
            Token::Folder {
                value: String::from("folder2"),
            },
            Token::Slash,
            Token::File {
                value: String::from("file"),
            },
            Token::Dot,
            Token::Extension {
                value: String::from("png"),
            },
        ];

        let command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn tokenize_multiple_commands() {
        let input = "cd folder1/folder2 && touch file.png";

        let expected_tokens = vec![
            Token::Command {
                value: String::from("cd"),
            },
            Token::Folder {
                value: String::from("folder1"),
            },
            Token::Slash,
            Token::Folder {
                value: String::from("folder2"),
            },
            Token::Command {
                value: String::from("touch"),
            },
            Token::File {
                value: String::from("file"),
            },
            Token::Dot,
            Token::Extension {
                value: String::from("png"),
            },
        ];

        let command_lexer = Lexer::new(input);
        let tokens = command_lexer.tokenize();

        assert_eq!(tokens, expected_tokens);
    }
}
