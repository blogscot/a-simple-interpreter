#[macro_use]
extern crate mopa;

mod interpreter;
mod lexer;
mod node;
mod parser;
mod token;

use interpreter::Interpreter;

fn main() {
    let interpreter = Interpreter::new("82 + 2");
    println!("{}", interpreter.evaluate());
}
