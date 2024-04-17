mod tokenizer;
mod parser;
mod checker;
mod ast;
mod display_impls;

use crate::tokenizer::*;
use crate::parser::*;
use crate::checker::*;

fn main() {
    let test_prog = std::fs::read_to_string("prog.lang").unwrap();

    let mut scanner = Scanner::new(test_prog.to_string());
    scanner.tokenize();
    // println!("{}\n", scanner.tokens);

    let mut parser = Parser::new(scanner.tokens.list);
    let program = parser.parse_program();
    println!("{}", &program);

    println!("----- After Inference -----");

    let program = TypeChecker::new().infer(program);
    println!("{}", program);
}
