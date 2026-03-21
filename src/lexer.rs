#[derive(Debug, PartialEq)]
pub enum Token {
    At,
    Identifier(String), // autor, title, etc.
    LParen,             // (
    Value(String),      // value inside ().
    RParen,             // )
    Eof,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a>{
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.next() {
            Some('@') => Token::At,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some(c) if c.is_alphabetic() => self.lex_identifier(c),
            Some('/') => self.lex_path(), // Special case for paths starting with /
            Some(c) => panic!("Unexpected character: {}", c),
            None => Token::Eof,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn lex_identifier(&mut self, first: char) -> Token {
        let mut ident = first.to_string();
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() {
                ident.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        Token::Identifier(ident)
    }

    fn lex_path(&mut self) -> Token {
        let mut path = "/".to_string();
        while let Some(&c) = self.input.peek() {
            // Paths contain dots, slashes, and alphanumeric chars
            if c.is_alphanumeric() || c == '.' || c == '/' {
                path.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        Token::Value(path)
    }
}
