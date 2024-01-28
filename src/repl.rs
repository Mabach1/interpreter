use std::io::{self, Write};

use crate::{
    lexer::Lexer,
    token::{self},
};

// later will expend on this
// guessing we'll need a whole struct for info about repl
pub fn repl_start() {
    loop {
        let mut buffer = String::new();

        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buffer).expect("Error reading from stdin");

        let mut lexer = Lexer::new(buffer);

        loop {
            let token = lexer.next_token();
            if token.token_type == token::TokenType::Eof {
                break;
            }
            println!("{:?}", token);
        }
    }
}
