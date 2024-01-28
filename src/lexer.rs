use crate::token::{self};

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

    fn is_letter(ch: char) -> bool {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.pos;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }
        return self.input[position..self.pos].to_string();
    }

    fn read_number(&mut self) -> String {
        let position = self.pos;
        while char::is_digit(self.ch, 10) {
            self.read_char();
        }
        return self.input[position..self.pos].to_string();
    }

    fn skip_whitespace(&mut self) {
        while char::is_whitespace(self.ch) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

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
            _ => {
                if Lexer::is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    token::Token::new(token::TokenType::lookup_indent(&identifier), identifier)
                } else if char::is_digit(self.ch, 10) {
                    let num_literal = self.read_number();
                    token::Token::new(token::TokenType::Int, num_literal)
                } else {
                    token::Token::new(token::TokenType::Illegal, self.ch.to_string())
                }
            }
        };

        // if we're to call the next character we would loose it
        // because of all the reading done in the read_identifier function
        if res.token_type == token::TokenType::Ident {
            return res;
        }

        self.read_char();

        return res;
    }
}
