mod preproc;
mod tokenizer;
mod parser;
mod checker;
mod ast;
mod interp;
mod display_impls;

use crate::preproc::*;
use crate::tokenizer::*;
use crate::parser::*;
use crate::checker::*;

fn main() {
    let test_prog = std::fs::read_to_string("prog.lang").unwrap();
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
