mod lexer;
mod parser;
mod checker;
mod ast;
mod builtins;
mod interp;
mod display_impls;
mod cps;
mod ast_to_cps;
mod passes;

use ast_to_cps::AstToCps;
use logos::Logos;
use passes::Shrinking;
use passes::TreePass;

use crate::lexer::*;
use crate::parser::*;
use crate::checker::*;

fn main() {
    // file name from first arg
    let file_name = std::env::args().nth(1).expect("no file name given");

    let prog = std::fs::read_to_string(file_name.clone()).unwrap();

    let lexer = Token::lexer(prog.as_str());
    // println!("----- Lexer -----");
    // for token in lexer.clone() {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    // println!("{}", &program);

    println!("----- Type Inference -----");

    let program = TypeChecker::new().infer(program);
    // println!("{}", program);

    println!("----- Tree Interpreter -----");

    let output = interp::eval_prog(&program);
    println!("{}", output);

    println!("----- CPS Lowering -----");
    let mut cps = AstToCps::convert(program);
    // println!("{:#}", cps);

    println!("----- Optimized CPS -----");

    let mut len = cps.len();
    let mut prev_len = len + 1;

    while len < prev_len {
        prev_len = len;

        let mut pass = Shrinking::new();
        cps = pass.apply(cps);

        len = cps.len();
    }

    println!("{:#}", cps);
    println!("{:#?}", cps.free());
}
