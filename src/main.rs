mod evaluator;
mod lexer;
mod parser;

use evaluator::evaluate;
use lexer::Lexer;
use parser::Parser;
use std::env;
// -----------------Evaluator-----------------------

fn main() {
    let arg: String = env::args().nth(1).unwrap();
    let l = Lexer::new(&arg);
    let tokens = l.tokenize();
    println!("Tokens:\n {:?}\n ", tokens);
    let ast = Parser::new(tokens).parse_expression();
    println!("AST: \n{:?}", ast);
    println!("El evaluador devuelve: {}", evaluate(&ast));
}
