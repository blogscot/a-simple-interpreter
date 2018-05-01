#[macro_use]
extern crate mopa;

mod lexer;
mod node;
mod parser;
mod token;

use node::Ast;
use parser::Parser;

fn main() {
    let mut parser = Parser::new("82 + 2");
    let nodes = parser.parse();
    let ast = Ast::new(nodes);

    println!("{}", ast.evaluate());
}
