use crate::token::{self, Token};

pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input,
            pos: 0,
            read_pos: 0,
            ch: '0',
        };

        lexer.read_char();
        return lexer;
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0';            
        } else {
            self.ch = self.input.chars().nth(self.read_pos).unwrap();
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let res = match self.ch {
            '=' => token::Token::new(token::TokenType::Assign, self.ch.to_string()),
            ';' => token::Token::new(token::TokenType::Semicolon, self.ch.to_string()),
            '(' => token::Token::new(token::TokenType::LeftParenthesis, self.ch.to_string()),
            ')' => token::Token::new(token::TokenType::RightParenthesis, self.ch.to_string()),
            ',' => token::Token::new(token::TokenType::Comma, self.ch.to_string()),
            '+' => token::Token::new(token::TokenType::Plus, self.ch.to_string()),
            '{' => token::Token::new(token::TokenType::LeftBrackets, self.ch.to_string()),
            '}' => token::Token::new(token::TokenType::RightBrackets, self.ch.to_string()),
            '\0' => token::Token::new(token::TokenType::Eof, self.ch.to_string()),
            _ => token::Token::new(token::TokenType::Illegal, String::new()),
        };

        self.read_char();
        return res;
    }
}
