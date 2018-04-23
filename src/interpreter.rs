use token::Token;
use token::TokenType::*;

#[derive(Clone)]
pub struct Interpreter {
  text: String,
  position: usize,
  current_token: Option<Token>,
}

impl Interpreter {
  pub fn new(text: String) -> Self {
    Interpreter {
      text,
      position: 0,
      current_token: None,
    }
  }
  fn get_next_token(&mut self) -> Option<Token> {
    let end_of_input = self.text.len() - 1;
    if self.position > end_of_input {
      return Some(Token { token_type: EOF });
    }
    let mut current_char = self.get_char(self.position);
    match current_char {
      char if char.is_digit(10) => {
        let mut digits = String::new();
        while current_char.is_digit(10) {
          if self.position == end_of_input {
            digits.push(current_char);
            break;
          }
          digits.push(current_char);
          self.position += 1;
          current_char = self.get_char(self.position);
        }
        Some(Token {
          token_type: Integer(digits.parse::<i32>().unwrap()),
        })
      }
      ' ' => {
        self.position += 1;
        self.get_next_token()
      }
      '+' => {
        self.position += 1;
        Some(Token { token_type: Plus })
      }
      '-' => {
        self.position += 1;
        Some(Token { token_type: Minus })
      }
      '*' => {
        self.position += 1;
        Some(Token {
          token_type: Multiply,
        })
      }
      '/' => {
        self.position += 1;
        Some(Token { token_type: Divide })
      }
      _ => panic!(format!("Invalid token found: {}", current_char)),
    }
  }
  fn get_current_token(&self) -> Token {
    self.clone().current_token.unwrap()
  }
  fn get_char(&self, position: usize) -> char {
    self.text.as_bytes()[position] as char
  }
  ///
  /// Verifies the token type matches the current token type.
  /// If valid the next token is saved.
  ///
  fn consume(&mut self, token: &Token) {
    let current_token = self.get_current_token();
    if current_token.token_type == token.token_type {
      self.current_token = self.get_next_token();
    } else {
      panic!("Token error: next!")
    }
  }
  pub fn expr(&mut self) -> i32 {
    let mut left = 0;
    let mut right = 0;
    self.current_token = self.get_next_token();
    let mut token = self.get_current_token();

    if let Integer(value) = token.token_type {
      left = value;
      self.consume(&token);
    }
    token = self.get_current_token();
    let operator = token.clone().token_type;
    self.consume(&token);

    token = self.get_current_token();
    if let Integer(value) = token.token_type {
      right = value;
      self.consume(&token);
    }

    match operator {
      Plus => left + right,
      Minus => left - right,
      Multiply => left * right,
      Divide => left / right,
      _ => panic!("Unknown operator encountered!"),
    }
  }
}
