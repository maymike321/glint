pub mod lexing;
pub mod token;

use crate::lexing::lexer::Lexer;

fn main() {
    let data = std::fs::read_to_string("C:\\git\\glint\\program.glint").unwrap_or("".to_string());
    let mut lexer: Lexer = Lexer::new(&data);
    let tokens = lexer.scan();
    for token in tokens {
        println!("{:?} - {} on line {}", token.token_type, token.string, token.line);
    }
}
