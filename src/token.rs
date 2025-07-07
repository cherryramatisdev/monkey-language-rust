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
    Minus,
    Bang,
    Asterisk,
    Slash,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LessThan,
    GreaterThan,
    Function,
    Let,
}
