use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
  Program,
  Procedure,
  Begin,
  End,
  Var,
  Integer,
  Real,
  IntegerConst(String),
  RealConst(String),
  Id(String),
  Colon,
  Comma,
  Semi,
  Period,
  Plus,
  Minus,
  Multiply,
  IntegerDivision,
  RealDivision,
  LParen,
  RParen,
  Assign,
  EOF,
}

impl<'a> fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output = match self {
      Token::Program => "PROGRAM",
      Token::Procedure => "PROCEDURE",
      Token::Begin => "BEGIN",
      Token::End => "END",
      Token::Var => "VAR",
      Token::Integer => "INTEGER",
      Token::Real => "REAL",
      Token::IntegerConst(value) => value,
      Token::RealConst(value) => value,
      Token::Id(name) => name,
      Token::Colon => ":",
      Token::Comma => ",",
      Token::Semi => ";",
      Token::Period => ".",
      Token::Plus => "+",
      Token::Minus => "-",
      Token::Multiply => "*",
      Token::IntegerDivision => "DIV",
      Token::RealDivision => "/",
      Token::LParen => "(",
      Token::RParen => ")",
      Token::Assign => ":=",
      Token::EOF => "EOF",
    };
    write!(f, "{}", output)
  }
}
