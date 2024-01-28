use lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let input = String::from(
        r#"
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
    "#,
    );

    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();

        if token.token_type == token::TokenType::Eof {
            break;
        }

        println!("{:?}", token);
    }
}
