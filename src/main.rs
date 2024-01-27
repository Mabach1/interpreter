use lexer::Lexer;

mod token;
mod lexer;

fn main() {
    let input = String::from("=+(){},;");
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();

        if token.token_type == token::TokenType::Eof {
            break;
        }

        println!("{:?}", token);
    }
}
