use crate::token;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    LetStatement(Option<token::Token>),
    Statement(token::Token),
    Expression(token::Token),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub nodes: Vec<Node>,
}
