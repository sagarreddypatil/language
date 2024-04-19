mod preproc;
mod tokenizer;
mod parser;
mod checker;
mod ast;
mod builtins;
mod interp;
mod display_impls;

use crate::preproc::*;
use crate::tokenizer::*;
use crate::parser::*;
use crate::checker::*;

fn main() {
    // file name from first arg
    let file_name = std::env::args().nth(1).expect("no file name given");

    let test_prog = std::fs::read_to_string(file_name).unwrap();
    let prog = preprocess(test_prog);

    let mut scanner = Scanner::new(prog.to_string());
    scanner.tokenize();

    let mut parser = Parser::new(scanner.tokens.list);
    let program = parser.parse_program();
    println!("{}", &program);

    println!("----- After Inference -----");

    let program = TypeChecker::new().infer(program);
    println!("{}", program);

    println!("----- Output -----");

    let output = interp::eval_prog(&program);
    println!("{}", output);
}
