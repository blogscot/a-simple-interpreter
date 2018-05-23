#[macro_use]
extern crate mopa;

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod symbols;
mod utils;

use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use utils::number::Number::Nil;

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
  match interpreter.interpret() {
    Ok(Nil) => println!("Success!"),
    Ok(value) => println!("Program terminated with value: {:?}", value),
    Err(msg) => println!("!!!Error!!!: {}", msg),
  }
  Ok(())
}
