#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl <'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        lexer.read_char()
    }

    pub fn read_char(self) -> Self {
        Self {
            input: self.input,
            position: self.read_position,
            read_position: self.read_position + 1,
            ch: match self.input.chars().nth(self.read_position) {
                Some(ch) => ch,
                None => ' ', 
            }
        }
    }
}

mod test {
    use super::Lexer;

    #[test]
    fn test_read_char() {
        let content = String::from("adsf asdf asdf");
        let lexer = Lexer::new(&content);

        println!("{:?}", lexer);

        let lexer = lexer.read_char();

        println!("{:?}", lexer);
    }
}
