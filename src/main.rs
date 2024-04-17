mod tokenizer;
mod parser;
mod checker;
mod ast;
mod display_impls;

use crate::tokenizer::*;
use crate::parser::*;
use crate::checker::*;

fn main() {
    let test_prog = "data Maybe = Some(Int) | None

let div = fn(a, b)
    match b {
        0: None
        _: Some(a / b)
    }

let gcd = fn(a, b) match b {
    0 : a
    _ : gcd(b, a % b)
}

gcd(10, 5)";

    let mut scanner = Scanner::new(test_prog.to_string());
    scanner.tokenize();
    // println!("{}\n", scanner.tokens);

    let mut parser = Parser::new(scanner.tokens.list);
    let program = parser.parse_program();
    println!("{}", program);

    let substs = TypeChecker::new().infer(program);
    println!("{}", substs);
}
