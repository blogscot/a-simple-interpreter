use token::Token;
use token::TokenType;
use token::TokenType::*;

use lexer::Lexer;
use node::{BinOpNode, Node, NumNode};

#[derive(Clone)]
pub struct Parser {
  lexer: Lexer,
  current_token: Option<Token>,
}

impl Parser {
  pub fn new(text: &str) -> Self {
    let mut lexer = Lexer::new(&text);
    let current_token = lexer.get_next_token();

    Parser {
      lexer,
      current_token,
    }
  }
  fn get_current_token(&self) -> Token {
    self.current_token.clone().unwrap()
  }
  fn get_token_type(&self) -> TokenType {
    self.get_current_token().get_type()
  }
  ///
  /// Verifies the token type matches the current token type.
  /// If valid the next token is saved.
  ///
  fn consume(&mut self, token_type: &TokenType) {
    let current_token = self.get_current_token();

    if current_token.get_type() == *token_type {
      self.current_token = self.lexer.get_next_token();
    } else {
      panic!(format!("consume: token error: {}", current_token));
    }
  }
  fn factor(&mut self) -> Box<Node> {
    let token_type = self.get_token_type();

    if let Integer(value) = token_type {
      self.consume(&token_type);
      Box::new(NumNode::new(value))
    } else if let LParen = token_type {
      self.consume(&token_type);
      let result = self.expr();
      self.consume(&TokenType::RParen);
      result
    } else {
      panic!(format!("Invalid factor: {}", token_type));
    }
  }
  fn term(&mut self) -> Box<Node> {
    let mut node = self.factor();

    let mut token_type = self.get_token_type();
    while token_type == Multiply || token_type == Divide {
      token_type = self.get_token_type();

      if token_type == Multiply || token_type == Divide {
        self.consume(&token_type);
        node = Box::new(BinOpNode::new(self.factor(), node, token_type));
      }
    }
    node
  }
  fn expr(&mut self) -> Box<Node> {
    let mut node = self.term();

    let mut token_type = self.get_token_type();
    while token_type == Plus || token_type == Minus {
      token_type = self.get_token_type();

      if token_type == Plus || token_type == Minus {
        self.consume(&token_type);
        node = Box::new(BinOpNode::new(self.factor(), node, token_type));
      }
    }
    node
  }
  pub fn parse(&mut self) -> Box<Node> {
    self.expr()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_two_single_digit_numbers() {
    let mut interpreter = Interpreter::new("4 + 7".into());
    let result = interpreter.expr();

    // assert_eq!(result, 11);
  }

  #[test]
  #[ignore]
  fn subtract_two_single_digit_numbers() {
    let mut interpreter = Interpreter::new("4 - 7".into());
    let result = interpreter.expr();

    // assert_eq!(result, -3);
  }

  #[test]
  #[ignore]
  fn multiply_two_single_digit_numbers() {
    let mut interpreter = Interpreter::new("4 * 7".into());
    let result = interpreter.expr();

    // assert_eq!(result, 28);
  }

  #[test]
  #[ignore]
  fn divide_two_single_digit_numbers() {
    let mut interpreter = Interpreter::new("10 / 3".into());
    let result = interpreter.expr();

    // assert_eq!(result, 3);
  }

  #[test]
  #[ignore]
  fn add_multiple_digit_numbers() {
    let mut interpreter = Interpreter::new("101 + 99".into());
    let result = interpreter.expr();

    // assert_eq!(result, 200);
  }

  #[test]
  #[ignore]
  fn subtract_multiple_digit_numbers() {
    let mut interpreter = Interpreter::new("1234 - 134".into());
    let result = interpreter.expr();

    // assert_eq!(result, 1100);
  }

  #[test]
  #[ignore]
  fn add_multiple_numbers() {
    let mut interpreter = Interpreter::new("1 + 2 + 3 + 4 + 5".into());
    let result = interpreter.expr();

    // assert_eq!(result, 15);
  }

  #[test]
  #[ignore]
  fn add_and_subtract_multiple_numbers() {
    let mut interpreter = Interpreter::new("1 + 2 - 3 + 4 - 5".into());
    let result = interpreter.expr();

    // assert_eq!(result, -1);
  }

  #[test]
  #[ignore]
  fn muliply_and_divide_multiple_numbers() {
    let mut interpreter = Interpreter::new("10 * 20 / 2 / 10".into());
    let result = interpreter.expr();

    // assert_eq!(result, 10);
  }

  #[test]
  #[ignore]
  fn evaluate_multiterm_expression_contain_parens() {
    let mut interpreter = Interpreter::new("6 * (3 + 7) / 2".into());
    let result = interpreter.expr();

    // assert_eq!(result, 30);
  }
}
