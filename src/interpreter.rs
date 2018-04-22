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

    let current_char = self.text.as_bytes()[self.position] as char;
    match current_char {
      char if char.is_digit(10) => {
        self.position += 1;
        Some(Token {
          token_type: Integer(current_char.to_digit(10).unwrap() as i32),
        })
      }
      ' ' => {
        self.position += 1;
        return self.get_next_token();
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
  fn eat(&mut self, token: Token) {
    let current_token = self.clone().current_token.unwrap();
    if current_token.token_type == token.token_type {
      self.current_token = self.get_next_token();
    } else {
      panic!("Token error: eat!")
    }
  }
  pub fn expr(&mut self) -> i32 {
    self.current_token = self.get_next_token();
    let mut left = 0;
    let mut right = 0;
    let mut operator = "unknown";

    let token = self.clone().current_token.unwrap();
    if let Integer(value) = token.token_type {
      left = value;
      self.eat(token);
    }

    let token = self.clone().current_token.unwrap();
    if token.token_type == Plus {
      operator = "plus";
      self.eat(token);
    } else if token.token_type == Minus {
      operator = "minus";
      self.eat(token);
    }

    let token = self.clone().current_token.unwrap();
    if let Integer(value) = token.token_type {
      right = value;
      self.eat(token);
    }

    match operator.as_ref() {
      "plus" => left + right,
      "minus" => left - right,
      _ => panic!("Unknown operator encountered!"),
    }
  }
}
