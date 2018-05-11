#[macro_use]
extern crate mopa;

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod interpreter;
mod lexer;
mod node;
mod number;
mod parser;
mod token;
mod visitor;

use interpreter::Interpreter;

fn main() {
  let mut interpreter = Interpreter::new(
    r#"
    PROGRAM empty;
    BEGIN 
    END."#,
  );
  println!("{:?}", interpreter.interpret());
}
