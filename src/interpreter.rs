use token::Token;
use token::TokenType;
use token::TokenType::*;

use lexer::Lexer;

#[derive(Clone)]
pub struct Interpreter {
  lexer: Lexer,
  current_token: Option<Token>,
}

impl Interpreter {
  pub fn new(text: String) -> Self {
    let mut lexer = Lexer::new(&text);
    let current_token = lexer.get_next_token();

    Interpreter {
      lexer,
      current_token,
    }
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
      self.current_token = self.lexer.get_next_token();
    } else {
      panic!(format!("consume: token error: {}", current_token));
    }
  }
  fn factor(&mut self) -> i32 {
    let token_type = self.get_token_type();

    if let Integer(value) = token_type {
      self.consume(&token_type);
      value
    } else {
      panic!(format!("Invalid factor: {}", token_type));
    }
  }
  fn term(&mut self) -> i32 {
    let mut result = self.factor();

    let mut token_type = self.get_token_type();
    while token_type == Multiply || token_type == Divide {
      token_type = self.get_token_type();

      if token_type == Multiply {
        self.consume(&token_type);
        result *= self.factor()
      } else if token_type == Divide {
        self.consume(&token_type);
        result /= self.factor()
      }
    }
    result
  }
  pub fn expr(&mut self) -> i32 {
    let mut result = self.term();

    let mut token_type = self.get_token_type();
    while token_type == Plus || token_type == Minus {
      token_type = self.get_token_type();

      if token_type == Plus {
        self.consume(&token_type);
        result += self.term()
      } else if token_type == Minus {
        self.consume(&token_type);
        result -= self.term()
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_two_single_digit_numbers() {
    let mut interpreter = Interpreter::new("4 + 7".into());
    let result = interpreter.expr();

    assert_eq!(result, 11);
  }

  #[test]
  fn subtract_two_single_digit_numbers() {
    let mut interpreter = Interpreter::new("4 - 7".into());
    let result = interpreter.expr();

    assert_eq!(result, -3);
  }

  #[test]
  fn multiply_two_single_digit_numbers() {
    let mut interpreter = Interpreter::new("4 * 7".into());
    let result = interpreter.expr();

    assert_eq!(result, 28);
  }

  #[test]
  fn divide_two_single_digit_numbers() {
    let mut interpreter = Interpreter::new("10 / 3".into());
    let result = interpreter.expr();

    assert_eq!(result, 3);
  }

  #[test]
  fn add_multiple_digit_numbers() {
    let mut interpreter = Interpreter::new("101 + 99".into());
    let result = interpreter.expr();

    assert_eq!(result, 200);
  }

  #[test]
  fn subtract_multiple_digit_numbers() {
    let mut interpreter = Interpreter::new("1234 - 134".into());
    let result = interpreter.expr();

    assert_eq!(result, 1100);
  }

  #[test]
  fn add_multiple_numbers() {
    let mut interpreter = Interpreter::new("1 + 2 + 3 + 4 + 5".into());
    let result = interpreter.expr();

    assert_eq!(result, 15);
  }

  #[test]
  fn add_and_subtract_multiple_numbers() {
    let mut interpreter = Interpreter::new("1 + 2 - 3 + 4 - 5".into());
    let result = interpreter.expr();

    assert_eq!(result, -1);
  }

  #[test]
  fn muliply_and_divide_multiple_numbers() {
    let mut interpreter = Interpreter::new("10 * 20 / 2 / 10".into());
    let result = interpreter.expr();

    assert_eq!(result, 10);
  }
}
