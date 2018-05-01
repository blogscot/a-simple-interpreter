#[macro_use]
extern crate mopa;

mod interpreter;
mod lexer;
mod node;
mod token;

use interpreter::Interpreter;
use node::Ast;

fn main() {
    let mut interpreter = Interpreter::new("82 + 2");
    let nodes = interpreter.expr();
    let ast = Ast::new(nodes);

    println!("{}", ast.evaluate());
}
