use crate::token::{self, Token};

#[derive(Debug, PartialEq)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char()
    }

    pub fn read_char(self) -> Self {
        Self {
            input: self.input,
            position: self.read_position,
            read_position: self.read_position + 1,
            ch: match self.input.chars().nth(self.read_position) {
                Some(ch) => Some(ch),
                None => None,
            },
        }
    }

    pub fn eat_whitespace(self) -> Self {
        match self.ch {
            Some(' ' | '\t' | '\n' | '\r') => self.read_char().eat_whitespace(),
            _ => self,
        }
    }

    pub fn next_token(self) -> (Self, token::Token) {
        let lexer = self.eat_whitespace();

        match lexer.ch {
            Some(';') => {
                let (lexer, _) = lexer.read_line();
                lexer.next_token()
            }
            _ => (
                lexer.read_char(),
                Token {
                    token_type: token::TokenType::Invalid,
                    literal: String::from(""),
                },
            ),
        }
    }

    pub fn read_line(self) -> (Self, String) {
        match self.read_char() {
            Lexer {
                input,
                position,
                read_position,
                ch: Some('\n') | None,
            } => (
                Lexer {
                    input,
                    position,
                    read_position,
                    ch: Some('\n'),
                },
                String::from(""),
            ),
            lexer => {
                let character = String::from(lexer.ch.unwrap());
                let (lexer, comment) = lexer.read_line();
                (lexer, format!("{}{}", character, comment))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_read_char() {
        let content = String::from("Lorem Ipsum");
        let lexer = Lexer::new(&content);
        assert_eq!(
            lexer,
            Lexer {
                input: &content,
                position: 0,
                read_position: 1,
                ch: Some('L')
            }
        );

        let lexer = lexer.read_char();
        assert_eq!(
            lexer,
            Lexer {
                input: &content,
                position: 1,
                read_position: 2,
                ch: Some('o')
            }
        );
    }

    #[test]
    fn test_skip_whitespace() {
        let content = String::from(" \n \t \r Lorem Ipsum");
        let lexer = Lexer::new(&content);

        let lexer = lexer.eat_whitespace();
        assert_eq!(
            lexer,
            Lexer {
                input: &content,
                position: 7,
                read_position: 8,
                ch: Some('L')
            }
        );
    }
}
