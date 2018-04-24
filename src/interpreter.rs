use token::Token;
use token::TokenType;
use token::TokenType::*;

#[derive(Clone)]
pub struct Interpreter {
  text: String,
  position: usize,
  current_token: Option<Token>,
  current_char: Option<char>,
}

impl Interpreter {
  pub fn new(text: String) -> Self {
    Interpreter {
      text: text.clone(),
      position: 0,
      current_token: None,
      current_char: Some(text.as_bytes()[0] as char),
    }
  }
  fn advance(&mut self) {
    self.position += 1;
    if self.position > self.text.len() - 1 {
      self.current_char = None
    } else {
      self.current_char = Some(self.text.as_bytes()[self.position] as char)
    }
  }
  fn skip_whitespace(&mut self) {
    while self.current_char != None && self.current_char.unwrap().is_whitespace() {
      self.advance()
    }
  }
  fn integer(&mut self) -> Option<Token> {
    let mut digits = String::new();
    while self.current_char != None && self.current_char.unwrap().is_digit(10) {
      digits.push(self.current_char.unwrap());
      self.advance();
    }
    Some(Token {
      token_type: Integer(digits.parse::<i32>().unwrap()),
    })
  }
  fn get_next_token(&mut self) -> Option<Token> {
    while self.current_char != None {
      return match self.current_char.unwrap() {
        char if char.is_whitespace() => {
          self.skip_whitespace();
          continue;
        }
        char if char.is_digit(10) => self.integer(),
        '+' => {
          self.advance();
          Some(Token { token_type: Plus })
        }
        '-' => {
          self.advance();
          Some(Token { token_type: Minus })
        }
        '*' => {
          self.advance();
          Some(Token {
            token_type: Multiply,
          })
        }
        '/' => {
          self.advance();
          Some(Token { token_type: Divide })
        }
        _ => panic!("Unknown token found!"),
      };
    }
    Some(Token { token_type: EOF })
  }
  fn get_current_token(&self) -> Token {
    self.clone().current_token.unwrap()
  }
  fn get_token_type(&self) -> TokenType {
    self.get_current_token().token_type
  }
  ///
  /// Verifies the token type matches the current token type.
  /// If valid the next token is saved.
  ///
  fn consume(&mut self, token_type: &TokenType) {
    let current_token = self.get_current_token();
    if current_token.token_type == *token_type {
      self.current_token = self.get_next_token();
    } else {
      panic!(format!("consume: token error: {}", current_token));
    }
  }
  fn term(&mut self) -> i32 {
    let token = self.clone().current_token.unwrap();
    match token.token_type {
      Integer(value) => {
        self.consume(&token.token_type);
        value
      }
      _ => panic!(format!("Unexpected term found: {}", token.token_type)),
    }
  }
  pub fn expr(&mut self) -> i32 {
    // Get first token
    self.current_token = self.get_next_token();

    let mut result = self.term();
    let mut token_type = self.get_token_type();
    while token_type == Plus || token_type == Minus || token_type == Multiply
      || token_type == Divide
    {
      token_type = self.get_token_type();
      if token_type == Plus {
        self.consume(&token_type);
        result += self.term()
      } else if token_type == Minus {
        self.consume(&token_type);
        result -= self.term()
      } else if token_type == Multiply {
        self.consume(&token_type);
        result *= self.term()
      } else if token_type == Divide {
        self.consume(&token_type);
        result /= self.term()
      }
    }
    result
  }
}
