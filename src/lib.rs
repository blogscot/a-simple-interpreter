#[macro_use]
extern crate mopa;

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod ast;
pub mod interpreter;
mod lexer;
mod parser;
mod symbols;
pub mod utils;
