use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
  Integer(i32),
  Plus,
  Minus,
  Multiply,
  Divide,
  LParen,
  RParen,
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
      TokenType::LParen => "(".into(),
      TokenType::RParen => ")".into(),
      TokenType::EOF => "EOF".into(),
    };
    write!(f, "{}", output)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token(TokenType);

impl Token {
  pub fn new(token_type: TokenType) -> Self {
    Token(token_type)
  }
  pub fn get_type(&self) -> TokenType {
    self.0.clone()
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Token({})", self.0)
  }
}
