use token;
use token::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            Some('+') => Token::Plus,
            Some(';') => Token::Semicolon,

            Some(c @ _) => {
                if c.is_alphabetic() {
                    let ident = self.read_ident(c);
                    token::lookup_ident(&ident)
                } else if c.is_numeric() {
                    let number = self.read_number(c);
                    Token::Number(number)
                } else {
                    Token::Illegal
                }
            },

            None => Token::EOF
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, c: char) -> bool {
        match self.peek_char() {
            Some(&pc) => pc == c,
            None => false
        }
    }

    fn peek_is_alphabetic(&mut self) -> bool {
        match self.peek_char() {
            Some(&c) => c.is_alphabetic(),
            None => false
        }
    }

    fn peek_is_numeric(&mut self) -> bool {
        match self.peek_char() {
            Some(&c) => c.is_numeric(),
            None => false
        }
    }

    fn read_ident(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_alphabetic() {
            ident.push(self.read_char().unwrap());
        }

        ident
    }

    fn read_number(&mut self, first: char) -> u64 {
        let mut number = String::new();
        number.push(first);

        while self.peek_is_numeric() {
            number.push(self.read_char().unwrap());
        }

        number.parse::<u64>().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }
}
