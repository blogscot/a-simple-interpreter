use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
  Integer(i32),
  Plus,
  Minus,
  Multiply,
  Divide,
  LParen,
  RParen,
  EOF,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output = match self {
      Token::Integer(value) => format!("Integer({})", value),
      Token::Plus => "Plus".into(),
      Token::Minus => "Minus".into(),
      Token::Multiply => "Multiply".into(),
      Token::Divide => "Divide".into(),
      Token::LParen => "(".into(),
      Token::RParen => ")".into(),
      Token::EOF => "EOF".into(),
    };
    write!(f, "{}", output)
  }
}
