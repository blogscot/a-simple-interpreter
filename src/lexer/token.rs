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
      Token::Program => "PROGRAM".into(),
      Token::Procedure => "PROCEDURE".into(),
      Token::Begin => "BEGIN".into(),
      Token::End => "END".into(),
      Token::Var => "VAR".into(),
      Token::Integer => "INTEGER".into(),
      Token::Real => "REAL".into(),
      Token::IntegerConst(value) => value.to_string(),
      Token::RealConst(value) => value.to_string(),
      Token::Id(name) => name.to_string(),
      Token::Colon => ":".into(),
      Token::Comma => ",".into(),
      Token::Semi => ";".into(),
      Token::Period => ".".into(),
      Token::Plus => "+".into(),
      Token::Minus => "-".into(),
      Token::Multiply => "*".into(),
      Token::IntegerDivision => "DIV".into(),
      Token::RealDivision => "/".into(),
      Token::LParen => "(".into(),
      Token::RParen => ")".into(),
      Token::Assign => ":=".into(),
      Token::EOF => "EOF".into(),
    };
    write!(f, "{}", output)
  }
}
