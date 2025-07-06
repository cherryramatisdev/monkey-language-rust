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

        return lexer;
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

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some('=') => Token::Assign,
            Some('+') => Token::Plus,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some(_) => Token::Illegal,
            None => Token::EOF
        };

        self.read_char();

        return token;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_next_token_simple_symbols() {
        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(String::from("=+(){},;"));

        for test in tests {
            let token = lexer.next_token();

            assert_eq!(token, test);
        }
    }
}
