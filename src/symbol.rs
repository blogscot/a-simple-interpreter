use std::fmt;
use token::Token;

pub trait Symbolize {
  fn new(name: &str, kind: &BuiltInSymbol) -> Symbol;
  fn build(name: Token) -> BuiltInSymbol;
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltInSymbol {
  pub name: Token,
}

impl Symbolize for BuiltInSymbol {
  fn new(_name: &str, _kind: &BuiltInSymbol) -> Symbol {
    panic!("Error! Use build() to construct built-in types.");
  }
  fn build(name: Token) -> BuiltInSymbol {
    match name {
      Token::Integer | Token::Real => BuiltInSymbol { name },
      _ => panic!("Invalid symbol value found {}", name),
    }
  }
}

impl fmt::Display for BuiltInSymbol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
  pub name: String,
  pub kind: BuiltInSymbol,
}

impl fmt::Display for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.name, self.kind)
  }
}

impl Symbolize for Symbol {
  fn new(name: &str, kind: &BuiltInSymbol) -> Symbol {
    Symbol {
      name: name.to_string(),
      kind: kind.clone(),
    }
  }
  fn build(_name: Token) -> BuiltInSymbol {
    panic!("Error! Only built-in types are constructed using build().");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn build_builtin_integer_type() {
    let ref int_type = BuiltInSymbol::build(Token::Integer);
    assert_eq!(Token::Integer, int_type.name);
  }

  #[test]
  fn build_builtin_real_type() {
    let ref real_type = BuiltInSymbol::build(Token::Real);
    assert_eq!(Token::Real, real_type.name);
  }

  #[test]
  #[should_panic]
  fn build_invalid_builtin_should_panic() {
    let ref real_type = BuiltInSymbol::build(Token::Colon);
    assert_eq!(Token::Real, real_type.name);
  }

  #[test]
  fn create_integer_variable() {
    let ref int_type = BuiltInSymbol::build(Token::Integer);
    let a = Symbol::new("a".into(), int_type);

    assert_eq!("a", a.name);
    assert_eq!(Token::Integer, a.kind.name);
    assert_eq!("a: INTEGER", a.to_string())
  }

  #[test]
  fn create_real_variable() {
    let ref real_type = BuiltInSymbol::build(Token::Real);
    let b = Symbol::new("b".into(), real_type);

    assert_eq!("b", b.name);
    assert_eq!(Token::Real, b.kind.name);
    assert_eq!("b: REAL", b.to_string())
  }

}
