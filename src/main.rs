use std::fs;

mod lexer;
mod token;

fn main() {
    let fbx_content = fs::read_to_string("cube.fbx").unwrap();
    let _lexer = lexer::Lexer::new(&fbx_content);
}
