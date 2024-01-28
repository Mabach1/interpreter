use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    Eof,

    // identifiers + literals
    Ident,
    Int,

    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Equal,
    NotEqual,

    // delimiters
    Comma,
    Semicolon,


    LeftParenthesis,
    RightParenthesis,
    LeftBrackets,
    RightBrackets,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl TokenType {
    pub fn lookup_indent(ident: &str) -> TokenType {
        let mut keywords = HashMap::new();
        keywords.insert("fn", TokenType::Function);
        keywords.insert("let", TokenType::Let);
        keywords.insert("true", TokenType::True);
        keywords.insert("false", TokenType::False);
        keywords.insert("if", TokenType::If);
        keywords.insert("else", TokenType::Else);
        keywords.insert("return", TokenType::Return);

        if keywords.contains_key(ident) {
            return *keywords.get(ident).unwrap();
        }

        TokenType::Ident
    }
}
