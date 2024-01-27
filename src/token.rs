#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lit: String,
}

impl Token {
    pub fn new(token_type: TokenType, lit: String) -> Self {
        Self { token_type, lit }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    Indent,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrackets,
    RightBrackets,
    Function,
    Let,
}
