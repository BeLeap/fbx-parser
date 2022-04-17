use std::fs;

mod token;
mod lexer;

fn main() {
    let fbx_content = fs::read_to_string("cube.fbx").unwrap();
    let _lexer = lexer::Lexer::new(&fbx_content);
}
