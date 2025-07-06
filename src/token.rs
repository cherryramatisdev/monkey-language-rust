#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Identifier(String),
    // NOTE: The parsing will be the job of the AST builder
    Integer(String),
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
