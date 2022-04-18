#[derive(Debug, PartialEq)]
pub enum TokenType {
    Invalid,
    EOF,
    Literal,
    Colon,
    LeftBrace,
    RightBrace,
    DoubleQuote,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
