use crate::ast;
use crate::lexer;
use crate::token;

struct WrongToken {
    expected: token::Token,
    got: token::Token,
}

enum ParserError {
    WrongToken(WrongToken),
}

pub struct Parser {
    lexer: lexer::Lexer,
    errors: Vec<ParserError>,
    cur_token: Option<token::Token>,
    peek_token: Option<token::Token>,
}

impl Parser {
    fn new(lexer: lexer::Lexer) -> Self {
        let mut parser = Self {
            lexer,
            cur_token: None,
            peek_token: None,
            errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_let_statement(&mut self) -> Option<ast::Node> {
        match &self.peek_token {
            Some(token::Token::Identifier(_)) => self.next_token(),
            token => {
                // TODO: how to deal with this unwrap better?
                self.errors.push(ParserError::WrongToken(WrongToken {
                    expected: token::Token::Identifier("".to_string()),
                    got: token.clone().unwrap(),
                }));
                return None;
            }
        };

        let statement = ast::Node::LetStatement(self.cur_token.clone());

        match &self.peek_token {
            Some(token::Token::Assign) => self.next_token(),
            token => {
                // TODO: how to deal with this unwrap better?
                self.errors.push(ParserError::WrongToken(WrongToken {
                    expected: token::Token::Identifier("".to_string()),
                    got: token.clone().unwrap(),
                }));
                return None;
            }
        };

        while self.cur_token != Some(token::Token::Semicolon) {
            self.next_token();
        }

        Some(statement)
    }

    fn parse_statement(&mut self) -> Option<ast::Node> {
        match self.cur_token {
            Some(token::Token::Let) => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program { nodes: vec![] };

        while self.cur_token != Some(token::Token::Eof) {
            let node = self.parse_statement();

            if let Some(n) = node {
                program.nodes.push(n);
            }

            self.next_token();
        }

        Some(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statement() {
        let input = String::from(
            r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
            "#,
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);

        assert!(program.is_some());
        assert_eq!(program.as_ref().map(|p| p.nodes.len()), Some(3));

        for (i, identifier) in ["x", "y", "foobar"].iter().enumerate() {
            if let Some(ref p) = program {
                let node = &p.nodes[i];

                assert_eq!(
                    node,
                    &ast::Node::LetStatement(Some(token::Token::Identifier(
                        identifier.to_string()
                    )))
                );
            }
        }
    }
}
