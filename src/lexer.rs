use crate::token::Token;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            let ch_at_pos = self.input.as_bytes()[self.read_position];
            self.ch = std::char::from_u32(ch_at_pos as u32);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while let Some(ch) = self.ch {
            if Lexer::is_letter(ch) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while let Some(ch) = self.ch {
            if ch.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[position..self.position].to_string()
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.ch == Some(' ')
            || self.ch == Some('\t')
            || self.ch == Some('\n')
            || self.ch == Some('\r')
        {
            self.read_char();
        }
    }

    fn consume_token(&mut self, token: Token) -> Token {
        self.read_char();

        token
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('=') => self.consume_token(Token::Assign),
            Some('+') => self.consume_token(Token::Plus),
            Some(',') => self.consume_token(Token::Comma),
            Some(';') => self.consume_token(Token::Semicolon),
            Some('(') => self.consume_token(Token::LeftParen),
            Some(')') => self.consume_token(Token::RightParen),
            Some('{') => self.consume_token(Token::LeftBrace),
            Some('}') => self.consume_token(Token::RightBrace),
            Some('a'..='z') => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    _ => Token::Identifier(identifier),
                }
            }
            Some('0'..='9') => Token::Integer(self.read_number()),
            Some(_) => self.consume_token(Token::Illegal),
            None => self.consume_token(Token::Eof),
        };

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_next_token_simple_symbols() {
        let mut tokens = vec![];
        let mut lexer = Lexer::new(String::from("=+(){},;"));

        let mut token = lexer.next_token();

        while token != Token::Eof {
            tokens.push(token);
            token = lexer.next_token();
        }

        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn test_next_token_keywords() {
        let mut tokens = vec![];
        let mut lexer =
            Lexer::new(std::fs::read_to_string("examples/var_functions.monkey").unwrap());

        let mut token = lexer.next_token();

        while token != Token::Eof {
            tokens.push(token);
            token = lexer.next_token();
        }

        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn test_next_token_keywords_with_symbols() {
        let mut tokens = vec![];
        let mut lexer =
            Lexer::new(std::fs::read_to_string("examples/var_functions_symbols.monkey").unwrap());

        let mut token = lexer.next_token();

        while token != Token::Eof {
            tokens.push(token);
            token = lexer.next_token();
        }

        insta::assert_debug_snapshot!(tokens);
    }
    }
}
