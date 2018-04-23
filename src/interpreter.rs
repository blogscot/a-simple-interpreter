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
    if self.position > self.text.len() - 1 {
      return Some(Token { token_type: EOF });
    }

    let mut current_char = self.text.as_bytes()[self.position] as char;
    match current_char {
      char if char.is_digit(10) => {
        let mut digits = String::new();
        while current_char.is_digit(10) {
          if self.position == self.text.len() - 1 {
            digits.push(current_char);
            break;
          }
          digits.push(current_char);
          self.position += 1;
          current_char = self.text.as_bytes()[self.position] as char;
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
      _ => panic!(format!("Invalid token found: {}", current_char)),
    }
  }
  fn get_current_token(&self) -> Token {
    self.clone().current_token.unwrap()
  }
  ///
  /// Verifies the token type matches the current token type.
  /// If valid the next token is saved.
  ///
  fn consume(&mut self, token: Token) {
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
    let mut operator = "unknown";
    self.current_token = self.get_next_token();
    let mut token = self.get_current_token();

    if let Integer(value) = token.token_type {
      left = value;
      self.consume(token);
    }
    token = self.get_current_token();
    if token.token_type == Plus {
      operator = "plus";
      self.consume(token);
    } else if token.token_type == Minus {
      operator = "minus";
      self.consume(token);
    }
    token = self.get_current_token();
    if let Integer(value) = token.token_type {
      right = value;
      self.consume(token);
    }

    match operator {
      "plus" => left + right,
      "minus" => left - right,
      _ => panic!("Unknown operator encountered!"),
    }
  }
}
