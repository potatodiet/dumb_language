#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    // commands
    Def,

    // primary
    Ident(String),
    Number(u64),

    // keywords
    Let,
    Semicolon,

    // operators
    Assign,
    Plus,
    Equal
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "let" => Token::Let,
        _ => Token::Ident(ident.to_string())
    }
}
