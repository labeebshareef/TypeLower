use super::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_numeric() {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        number
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char() {
            Some(':') => {
                self.advance();

                Token {
                    token_type: TokenType::Colon,
                    value: ":".to_string(),
                }
            }

            Some('=') => {
                self.advance();

                Token {
                    token_type: TokenType::Equal,
                    value: "=".to_string(),
                }
            }

            Some('+') => {
                self.advance();

                Token {
                    token_type: TokenType::Plus,
                    value: "+".to_string(),
                }
            }

            Some(';') => {
                self.advance();

                Token {
                    token_type: TokenType::Semicolon,
                    value: ";".to_string(),
                }
            }

            Some(ch) if ch.is_alphabetic() => {
                let ident = self.read_identifier();

                let token_type = match ident.as_str() {
                    "const" => TokenType::Const,
                    _ => TokenType::Identifier,
                };

                Token {
                    token_type,
                    value: ident,
                }
            }

            Some(ch) if ch.is_numeric() => {
                let number = self.read_number();

                Token {
                    token_type: TokenType::Number,
                    value: number,
                }
            }

            None => Token {
                token_type: TokenType::EOF,
                value: "".to_string(),
            },

            Some(ch) => {
                panic!("Unexpected character: {}", ch);
            }
        }
    }
}
