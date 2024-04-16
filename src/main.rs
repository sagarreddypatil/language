mod tokenizer;
mod parser;
mod ast;

use crate::tokenizer::*;

fn main() {
    let testProg = "
let gcd = fn(a, b)
    match b
        0 : a
        _ : gcd(b, a % b)

gcd(10, 5)
    ";

    let mut scanner = Scanner::new(testProg.to_string());
    scanner.tokenize();

    for token in scanner.tokens {
        println!("{:?}", token);
    }
}
