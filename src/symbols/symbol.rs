use lexer::token::Token;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltIn(Token);

impl BuiltIn {
  pub fn new(name: Token) -> Self {
    match name {
      Token::Integer | Token::Real => BuiltIn(name),
      _ => panic!("Invalid symbol value found {}", name),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
  BuiltInSymbol(BuiltIn),
  UserSymbol(String, BuiltIn),
  // ProcedureSymbol(String, Vec<HashMap<String, BuiltIn>>),
}

use self::Symbol::*;

pub struct SymbolTable {
  scope_name: String,
  scope_level: u32,
  symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
  pub fn new(scope_name: String, scope_level: u32) -> Self {
    let symbols = HashMap::new();
    let mut symbol_table = SymbolTable {
      scope_name,
      scope_level,
      symbols,
    };
    symbol_table.initialise_builtins();
    symbol_table
  }
  // Inserts a builtin type into the Symbol Table.
  pub fn set(&mut self, builtin: BuiltIn) {
    self
      .symbols
      .insert(builtin.0.to_string(), BuiltInSymbol(builtin));
  }
  // Returns the builtin type for the given token reference.
  pub fn get(&self, name: &Token) -> Symbol {
    self.lookup(&name.to_string()).unwrap()
  }
  // Inserts a user-defined type into the Symbol Table.
  pub fn insert(&mut self, symbol: Symbol) {
    if let Symbol::UserSymbol(key, _kind) = symbol.clone() {
      self.symbols.insert(key, symbol);
    } else {
      panic!(format!("Error, Invalid Symbol! {}", symbol));
    }
  }
  // Returns the matching symbol in the symbol table corresponding
  // to the given key.
  pub fn lookup(&self, key: &str) -> Option<Symbol> {
    match self.symbols.get(key) {
      None => None,
      Some(symbol) => Some(symbol.clone()),
    }
  }
  fn initialise_builtins(&mut self) {
    let int_type = BuiltIn::new(Token::Integer);
    let real_type = BuiltIn::new(Token::Real);
    self.set(int_type);
    self.set(real_type);
  }
}

impl fmt::Display for BuiltIn {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl fmt::Display for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BuiltInSymbol(symbol) => symbol.to_string(),
        UserSymbol(key, symbol) => format!("{}: {}", key, symbol),
      }
    )
  }
}

impl fmt::Display for SymbolTable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    println!("Symbol Table Info:");
    println!("Scope {}, Level {}", &self.scope_name, &self.scope_level);

    for (key, val) in &self.symbols {
      writeln!(f, "({} => {})", key, val).unwrap();
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn build_builtin_integer_type() {
    let int_type = BuiltIn::new(Token::Integer);
    assert_eq!(Token::Integer, int_type.0);
  }

  #[test]
  fn build_builtin_real_type() {
    let real_type = BuiltIn::new(Token::Real);
    assert_eq!(Token::Real, real_type.0);
  }

  #[test]
  #[should_panic]
  fn should_panic_build_invalid_builtin() {
    let real_type = BuiltIn::new(Token::Colon);
    assert_eq!(Token::Real, real_type.0);
  }

  #[test]
  fn create_integer_variable() {
    let a = UserSymbol("a".into(), BuiltIn::new(Token::Integer));

    if let UserSymbol(key, builtin) = a.clone() {
      assert_eq!("a", key);
      assert_eq!(Token::Integer, builtin.0);
      assert_eq!("a: INTEGER", a.to_string())
    }
  }

  #[test]
  fn create_real_variable() {
    let a = UserSymbol("a".into(), BuiltIn::new(Token::Real));

    if let UserSymbol(key, builtin) = a.clone() {
      assert_eq!("a", key);
      assert_eq!(Token::Real, builtin.0);
      assert_eq!("a: REAL", a.to_string())
    }
  }

  #[test]
  fn define_and_lookup_integer() {
    let mut symbol_table = SymbolTable::new("Global".into(), 1);

    if let BuiltInSymbol(builtin) = symbol_table.lookup("INTEGER").unwrap() {
      let int_variable = UserSymbol("a".into(), builtin);
      symbol_table.insert(int_variable);
      let symbol_lookup = symbol_table.lookup("a").unwrap();
      assert_eq!("a: INTEGER", symbol_lookup.to_string());
    }
  }

  #[test]
  fn define_and_lookup_real() {
    let mut symbol_table = SymbolTable::new("Global".into(), 1);

    if let BuiltInSymbol(builtin) = symbol_table.lookup("REAL").unwrap() {
      let real_variable = UserSymbol("x".into(), builtin);
      symbol_table.insert(real_variable);
      let symbol_lookup = symbol_table.lookup("x").unwrap();
      assert_eq!("x: REAL", symbol_lookup.to_string());
    }
  }

  #[test]
  fn lookup_unknown_variable() {
    let symbol_table = SymbolTable::new("Global".into(), 1);
    let symbol_lookup = symbol_table.lookup("x");
    assert_eq!(None, symbol_lookup);
  }

}
