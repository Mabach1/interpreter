use crate::repl::repl_start;

mod lexer;
mod token;
mod repl;

fn main() {
    println!("Hello this is the Monkey programming language!");
    println!("Feel free to type in commands");

    repl_start();
}
