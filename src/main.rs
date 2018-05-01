#[macro_use]
extern crate mopa;

mod ast;
mod lexer;
mod node;
mod parser;
mod token;

use ast::Ast;

fn main() {
    let ast = Ast::new("82 + 2");
    println!("{}", ast.evaluate());
}
