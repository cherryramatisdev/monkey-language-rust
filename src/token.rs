#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    Identifier(String),
    Integer(usize),
    Assign,
    Plus,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function,
    Let,
}
