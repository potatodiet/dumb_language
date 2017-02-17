pub mod lexer;
pub mod token;

use token::Token;
use lexer::Lexer;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("instructions.dula").expect("instructions.dula could not be read");
    let mut instructions = String::new();
    f.read_to_string(&mut instructions).unwrap();
    let mut lexer = Lexer::new(&instructions);

    loop {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        if tok == Token::EOF {
            break;
        }
    }
}
