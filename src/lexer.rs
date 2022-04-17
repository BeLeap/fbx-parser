#[derive(Debug, PartialEq)]
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

    pub fn skip_whitespace(self) -> Self {
        match self.ch {
            ' ' | '\t' | '\n' | '\r' => self.read_char().skip_whitespace(),
            _ => self,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;

    #[test]
    fn test_read_char() {
        let content = String::from("Lorem Ipsum");
        let lexer = Lexer::new(&content);
        assert_eq!(lexer, Lexer { input: &content, position: 0, read_position: 1, ch: 'L' });

        let lexer = lexer.read_char();
        assert_eq!(lexer, Lexer { input: &content, position: 1, read_position: 2, ch: 'o' });
    }

    #[test]
    fn test_skip_whitespace() {
        let content = String::from(" \n \t \r Lorem Ipsum");
        let lexer = Lexer::new(&content);

        let lexer = lexer.skip_whitespace();
        assert_eq!(lexer, Lexer { input: &content, position: 7, read_position: 8, ch: 'L' });
    }
}
