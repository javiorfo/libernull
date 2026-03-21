use crate::lexer::{Lexer, Token};

mod parser; 
mod lexer;

fn main() {
let input = "@autor(Author)";
    let mut lexer = Lexer::new(input);
    
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::Eof { break; }
    }
}
