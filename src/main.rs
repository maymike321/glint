pub mod lexing;
pub mod token;

use crate::lexing::lexer::Scanner;

fn main() {
    let data = std::fs::read_to_string("C:\\git\\glint\\program.glint").unwrap_or("".to_string());
    let mut scanner: Scanner = Scanner::new(&data);
    let tokens = scanner.scan();
    for token in tokens {
        println!("{:?} - {} on line {}", token.token_type, token.string, token.line);
    }
}
