mod lexer;
mod parser;
mod checker;
mod ast;
mod builtins;
mod interp;
mod display_impls;

use crate::lexer::*;
use crate::parser::*;
use crate::checker::*;

fn main() {
    // file name from first arg
    let file_name = std::env::args().nth(1).expect("no file name given");

    let prog = std::fs::read_to_string(file_name.clone()).unwrap();

    let mut scanner = Scanner::new(prog.to_string());
    scanner.tokenize();
    // println!("{}", scanner.tokens);

    let mut parser = Parser::new(scanner.tokens.list);
    let program = parser.parse_program();
    // println!("{}", &program);

    println!("----- Type Inference -----");

    let program = TypeChecker::new().infer(program);
    println!("{}", program);

    println!("----- Interpreter -----");

    let output = interp::eval_prog(&program);
    println!("{}", output);
}
