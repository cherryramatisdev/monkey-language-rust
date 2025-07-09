#![allow(dead_code)]

mod lexer;
mod token;

use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">> ";

fn main() {
    loop {
        let mut line = String::new();

        print!("{}", PROMPT);

        let _ = stdout().flush();
        stdin()
            .read_line(&mut line)
            .expect("Please type a valid string");

        match line.trim() {
            "exit" => {
                break;
            }
            _ => {
                let mut lexer = lexer::Lexer::new(line);

                let mut token = lexer.next_token();

                while token != token::Token::Eof {
                    println!("{:?}", token);
                    token = lexer.next_token();
                }
            }
        };
    }
}
