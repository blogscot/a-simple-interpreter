use std::fmt;

use std::collections::HashMap;
use symbol::{BuiltInSymbol, Symbol, Symbolize};
use token::Token;

#[derive(Debug, PartialEq)]
pub struct SymbolTable {
  builtins: HashMap<Token, BuiltInSymbol>,
  symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
  pub fn new() -> SymbolTable {
    let builtins: HashMap<Token, BuiltInSymbol> = HashMap::new();
    let symbols: HashMap<String, Symbol> = HashMap::new();
    let mut symbol_table = SymbolTable { builtins, symbols };
    symbol_table.initialise_builtins();
    symbol_table
  }
  fn set(&mut self, name: Token, symbol: &BuiltInSymbol) {
    self.builtins.insert(name, symbol.clone());
  }
  pub fn get(&self, name: &Token) -> BuiltInSymbol {
    self.builtins[name].clone()
  }
  pub fn define(&mut self, symbol: Symbol) {
    let key = symbol.clone().name;
    self.symbols.insert(key, symbol);
  }
  #[allow(dead_code)]
  pub fn lookup(&self, name: &str) -> Option<Symbol> {
    match self.symbols.get(name) {
      None => None,
      Some(symbol) => Some(symbol.clone()),
    }
  }
  fn initialise_builtins(&mut self) {
    let int_type = &BuiltInSymbol::build(Token::Integer);
    let real_type = &BuiltInSymbol::build(Token::Real);
    self.set(Token::Integer, int_type);
    self.set(Token::Real, real_type);
  }
}

impl fmt::Display for SymbolTable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    println!("Builtins");
    for (key, val) in &self.builtins {
      writeln!(f, "{} -> {}", key, val).unwrap();
    }
    println!("User Defined");
    for (key, val) in &self.symbols {
      writeln!(f, "{} -> {}", key, val).unwrap();
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use symbol::{BuiltInSymbol, Symbol};

  fn setup() -> (SymbolTable, BuiltInSymbol, BuiltInSymbol) {
    let symbol_table = SymbolTable::new();
    let int_type = symbol_table.get(&Token::Integer);
    let real_type = symbol_table.get(&Token::Real);
    (symbol_table, int_type, real_type)
  }

  #[test]
  fn define_and_lookup_integer() {
    let (mut symbol_table, int_type, _) = setup();

    let int_variable = Symbol::new("a", &int_type);
    symbol_table.define(int_variable);
    let symbol_lookup = symbol_table.lookup("a").unwrap();

    assert_eq!("a: INTEGER", symbol_lookup.to_string());
  }

  #[test]
  fn define_and_lookup_real() {
    let (mut symbol_table, _, real_type) = setup();

    let real_variable = Symbol::new("b", &real_type);
    symbol_table.define(real_variable);
    let symbol_lookup = symbol_table.lookup("b").unwrap();

    assert_eq!("b: REAL", symbol_lookup.to_string());
  }
}
