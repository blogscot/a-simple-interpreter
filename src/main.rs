#[macro_use]
extern crate mopa;

mod interpreter;
mod lexer;
mod node;
mod token;

use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new("7 + 3");
    interpreter.expr();

    // println!("{}", result);
}
