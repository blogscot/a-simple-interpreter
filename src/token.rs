use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
  Integer(i32),
  Plus,
  Minus,
  Multiply,
  Divide,
  EOF,
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output = match self {
      TokenType::Integer(value) => format!("Integer({})", value),
      TokenType::Plus => "Plus".into(),
      TokenType::Minus => "Minus".into(),
      TokenType::Multiply => "Multiply".into(),
      TokenType::Divide => "Divide".into(),
      TokenType::EOF => "EOF".into(),
    };
    write!(f, "{}", output)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Token({})", self.token_type)
  }
}
