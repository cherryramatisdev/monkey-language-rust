#[derive(Debug, PartialEq, Clone)]
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
    True,
    False,
    If,
    Else,
    Return,
    Equal,
    NotEqual,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}
