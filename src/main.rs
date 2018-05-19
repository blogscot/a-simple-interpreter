#[macro_use]
extern crate mopa;

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod evaluator;
mod interpreter;
mod lexer;
mod node;
mod number;
mod parser;
mod symbol;
mod symbol_table;
mod table_builder;
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
