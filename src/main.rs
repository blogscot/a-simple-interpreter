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

use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use interpreter::Interpreter;

fn read_from_file(filename: &str) -> std::io::Result<String> {
  let file = File::open(filename)?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;
  Ok(contents)
}

///
///  Read and Interpret a Pascal file.
///
/// Use:
///   cargo run <filename>
///
fn main() -> Result<(), Box<error::Error>> {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let source = read_from_file(&filename)?;

  println!("Processing file: {}", filename);
  let mut interpreter = Interpreter::new(source.as_str());
  if interpreter.interpret() == number::Number::Nil {
    println!("Success!");
  }
  Ok(())
}
