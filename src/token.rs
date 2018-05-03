use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Integer(i32),
  Plus,
  Minus,
  Multiply,
  Divide,
  LParen,
  RParen,
  Id(String),
  Assign,
  Semi,
  Period,
  Begin,
  End,
  EOF,
}

impl<'a> fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output = match self {
      Token::Integer(value) => format!("Integer({})", value),
      Token::Plus => "Plus".into(),
      Token::Minus => "Minus".into(),
      Token::Multiply => "Multiply".into(),
      Token::Divide => "Divide".into(),
      Token::LParen => "(".into(),
      Token::RParen => ")".into(),
      Token::Begin => "BEGIN".into(),
      Token::End => "END".into(),
      Token::Id(name) => name.to_string(),
      Token::Assign => ":=".into(),
      Token::Semi => ";".into(),
      Token::Period => ".".into(),
      Token::EOF => "EOF".into(),
    };
    write!(f, "{}", output)
  }
}
