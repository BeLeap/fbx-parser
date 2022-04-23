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

    pub fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }

    pub fn eat_whitespace(self) -> Self {
        match self.ch {
            Some(ch) => {
                if ch.is_whitespace() {
                    self.read_char().eat_whitespace()
                } else {
                    self
                }
            },
            _ => self,
        }
    }

    pub fn next_token(self) -> (Self, token::Token) {
        let lexer = self.eat_whitespace();

        match lexer.ch {
            Some(ch) => {
                match ch {
                    ';' => {
                        let (lexer, _) = lexer.consume_line();
                        lexer.next_token()
                    },
                    ':' => {
                        (lexer.read_char(), Token {
                            token_type: token::TokenType::Colon,
                            literal: ":".to_string(),
                        })
                    },
                    '{' => {
                        (lexer.read_char(), Token {
                            token_type: token::TokenType::LeftBrace,
                            literal: "{".to_string(),
                        })
                    },
                    '}' => {
                        (lexer.read_char(), Token {
                            token_type: token::TokenType::RightBrace,
                            literal: "}".to_string(),
                        })
                    },
                    '"' => {
                        let (lexer, string) = lexer.consume_string();
                        (lexer.read_char(), match string {
                                Some(string) => Token {
                                    token_type: token::TokenType::String,
                                    literal: format!("{}{}", ch, string)
                                },
                                None => Token {
                                    token_type: token::TokenType::Invalid,
                                    literal: "malformed string".to_string(),
                                }
                            }
                        )
                    },
                    ',' => {
                        (lexer.read_char(), Token {
                            token_type: token::TokenType::Comma,
                            literal: ",".to_string(),
                        })
                    }
                    ch => {
                        if ch.is_alphabetic() {
                            let (lexer, ident) = lexer.consume_ident();
                            (lexer.read_char(), Token {
                                token_type: token::TokenType::Ident,
                                literal: format!("{}{}", ch, ident),
                            })
                        } else if ch.is_numeric() || ch == '-' {
                            let (lexer, num_lit) = lexer.consume_number();
                            (lexer.read_char(), match num_lit {
                                Some(num_lit) => Token {
                                token_type: token::TokenType::Numeric,
                                literal: format!("{}{}", ch, num_lit),
                                },
                                None => Token {
                                    token_type: token::TokenType::Invalid,
                                    literal: "malformed number".to_string(),
                                }
                            })
                        } else {
                            (lexer.read_char(), Token {
                                token_type: token::TokenType::Invalid,
                                literal: format!("'{}' is invalid", ch),
                            })
                        }
                    }
                }
            },
            None => (
                lexer.read_char(),
                Token {
                    token_type: token::TokenType::EOF,
                    literal: "".to_string(),
                },
            ),
        }
    }

    pub fn consume_line(self) -> (Self, String) {
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
                "".to_string(),
            ),
            lexer => {
                let character = String::from(lexer.ch.unwrap());
                let (lexer, comment) = lexer.consume_line();
                (lexer, format!("{}{}", character, comment))
            }
        }
    }

    pub fn consume_ident(self) -> (Self, String) {
        match self.peek_char() {
            Some(ch) => {
                if ch.is_alphanumeric() {
                    let (lexer, remainder) = self.read_char().consume_ident();
                    (lexer, format!("{}{}", ch, remainder))
                } else {
                    (self, "".to_string())
                }
            },
            None => {
                (self, "".to_string())
            }
        }
    }

    pub fn consume_string(self) -> (Self, Option<String>) {
        match self.peek_char() {
            Some(ch) => {
                if ch == '"' {
                    let lexer = self.read_char();
                    (lexer, Some('"'.to_string()))
                } else {
                    let (lexer, remainder) = self.read_char().consume_string();
                    (lexer, match remainder {
                        Some(remainder) => Some(format!("{}{}", ch, remainder)),
                        None => None,
                    })
                }
            },
            None => {
                (self, None)
            }
        }
    }

    pub fn consume_number(self) -> (Self, Option<String>) {
        match self.peek_char() {
            Some(ch)=> {
                if ch.is_numeric() || ch == '.' {
                    let (lexer, remainder) = self.read_char().consume_number();
                    (lexer, match remainder {
                        Some(remainder) => Some(format!("{}{}", ch, remainder)),
                        None => None
                    })
                } else if ch == '\n' || ch == ',' || ch.is_whitespace() {
                    (self, Some(String::new()))
                } else {
                    (self, None)
                }
            }
            None => (self, None)
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
    fn test_eat_whitespace() {
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

    #[test]
    fn test_literal() {
        let content = r#"
            ;test comment 1
            ;test comment 2
            TestType: {
                TestSubType: "Test Property"
            }"#.to_string();
        let mut lexer = Lexer::new(&content);

        let desired_results = [
            Token { token_type: TokenType::Ident, literal: "TestType".to_string() },
            Token { token_type: TokenType::Colon, literal: ":".to_string() },
            Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Token { token_type: TokenType::Ident, literal: "TestSubType".to_string() },
            Token { token_type: TokenType::Colon, literal: ":".to_string() },
            Token { token_type: TokenType::String, literal: "\"Test Property\"".to_string() },
            Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            Token { token_type: TokenType::EOF, literal: "".to_string() },
        ];

        for desired_result in desired_results {
            let (new_l, token) = lexer.next_token();
            lexer = new_l;
            assert_eq!(desired_result, token);
        }
    }

    #[test]
    fn test_malformed_string() {
        let content = r#"TestType: {
                TestSubType: "Malformed String
            }"#;

        let lexer = Lexer::new(&content);

        fn exhaust(lexer: Lexer) -> bool {
            match lexer.next_token() {
                (_, Token {
                    token_type: TokenType::Invalid,
                    ..
                }) => true,
                (l, _) => exhaust(l)
            }
        }

        assert!(exhaust(lexer));
    }

    #[test]
    fn test_numeric() {
        let content = r#"
        TestType: {
            TestSubTypeOne: 100, 0, 0
            TestSubTypeTwo: 100.0
        }
        "#;

        let mut lexer = Lexer::new(&content);

        let desired_results = [
            Token { token_type: TokenType::Ident, literal: "TestType".to_string() },
            Token { token_type: TokenType::Colon, literal: ":".to_string() },
            Token { token_type: TokenType::LeftBrace, literal: "{".to_string() },
            Token { token_type: TokenType::Ident, literal: "TestSubTypeOne".to_string() },
            Token { token_type: TokenType::Colon, literal: ":".to_string() },
            Token { token_type: TokenType::Numeric, literal: "100".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::Numeric, literal: "0".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::Numeric, literal: "0".to_string() },
            Token { token_type: TokenType::Ident, literal: "TestSubTypeTwo".to_string() },
            Token { token_type: TokenType::Colon, literal: ":".to_string() },
            Token { token_type: TokenType::Numeric, literal: "100.0".to_string() },
            Token { token_type: TokenType::RightBrace, literal: "}".to_string() },
            Token { token_type: TokenType::EOF, literal: "".to_string() },
        ];

        for desired_result in desired_results {
            let (new_l, token) = lexer.next_token();
            lexer = new_l;
            println!("{:?}", token);
            assert_eq!(desired_result, token);
        }
    }

    #[test]
    fn test_malformed_number() {
        let content = r#"1.000_"#;
        
        let lexer = Lexer::new(&content);

        let (_, token) = lexer.next_token();
        assert_eq!(token, Token {
            token_type: TokenType::Invalid,
            literal: "malformed number".to_string()
        })
    }
}
