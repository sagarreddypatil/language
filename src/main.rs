mod tokenizer;
mod parser;
mod ast;

use crate::tokenizer::*;
use crate::parser::*;
use crate::ast::*;

fn main() {
    let test_prog = "data Maybe = Some(Int) | None
    type MyInt = Int
    
    let div = fn(a, b)
        match b
            0: None
            _: Some(a / b)
    
    let gcd = fn(a, b)
        match b
            0: a
            _: gcd(b, a % b)
    
    gcd(10, 5)";

    let mut scanner = Scanner::new(test_prog.to_string());
    scanner.tokenize();
    println!("{}\n", Tokens(&scanner.tokens));

    // for token in &scanner.tokens {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(scanner.tokens);
    let program = parser.parse_program();

    println!("{:?}", program);
}
