use std::fs;

use token::{Token, TokenType};

use crate::lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let fbx_content = fs::read_to_string("cube.fbx").unwrap();
    let lexer = lexer::Lexer::new(&fbx_content);

    fn exhaust(lexer: Lexer) -> Lexer {
        match lexer.next_token() {
            (l, Token { token_type: TokenType::EOF, .. }) => l,
            (l, t) => {
                println!("{:?}", t);
                exhaust(l)
            },
        }
    }

    exhaust(lexer);
}
