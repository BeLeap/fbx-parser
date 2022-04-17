#[derive(Debug, PartialEq)]
pub enum TokenType {
    NodeType,
    Property,
    Invalid,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
