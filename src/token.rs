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
    Ident,
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

impl TokenType {
    pub fn lookup_indent(ident: &str) -> TokenType {
        let mut keywords = HashMap::new();
        keywords.insert("fn", TokenType::Function);
        keywords.insert("let", TokenType::Let);

        if keywords.contains_key(ident) {
            return *keywords.get(ident).unwrap();
        }

        TokenType::Ident
    }
}
