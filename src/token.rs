use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Program,
  Begin,
  End,
  Var,
  Integer,
  Real,
  IntegerConst(i32),
  RealConst(f32),
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
      Token::Program => format!("PROGRAM"),
      Token::Begin => "BEGIN".into(),
      Token::End => "END".into(),
      Token::Var => format!("VAR"),
      Token::Integer => "INTEGER".into(),
      Token::Real => "REAL".into(),
      Token::IntegerConst(value) => format!("{}", value),
      Token::RealConst(value) => format!("{}", value),
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
