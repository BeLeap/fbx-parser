#[derive(Debug, PartialEq)]
pub enum TokenType {
    Invalid,
    EOF,
    Ident,
    Colon,
    LeftBrace,
    RightBrace,
    String,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
