#[macro_use]
extern crate mopa;

#[macro_use]
extern crate lazy_static;

mod interpreter;
mod lexer;
mod node;
mod parser;
mod token;
mod visitor;

use interpreter::Interpreter;

fn main() {
  let interpreter = Interpreter::new("82 + 2");
  println!("{}", interpreter.interpret());
}
