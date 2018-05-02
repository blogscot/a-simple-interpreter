use node::Node;
use parser::Parser;
use visitor::{Evaluator, NodeVisitor};

pub struct Interpreter {
  pub root_node: Box<Node>,
}

impl Interpreter {
  pub fn new(text: &str) -> Self {
    let mut parser = Parser::new(&text);
    let root_node = parser.parse();
    Interpreter { root_node }
  }
  pub fn evaluate(&self) -> i32 {
    self.accept(&Evaluator {})
  }
}

impl Node for Interpreter {
  fn accept(&self, visitor: &NodeVisitor) -> i32 {
    self.root_node.accept(visitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_two_single_digit_numbers() {
    let interpreter = Interpreter::new("4 + 7");
    assert_eq!(interpreter.evaluate(), 11);
  }

  #[test]
  fn subtract_two_single_digit_numbers() {
    let interpreter = Interpreter::new("4 - 7");
    assert_eq!(interpreter.evaluate(), -3);
  }

  #[test]
  fn multiply_two_single_digit_numbers() {
    let interpreter = Interpreter::new("4 * 7");
    assert_eq!(interpreter.evaluate(), 28);
  }

  #[test]
  fn divide_two_single_digit_numbers() {
    let interpreter = Interpreter::new("10 / 3");
    assert_eq!(interpreter.evaluate(), 3);
  }

  #[test]
  fn add_multiple_digit_numbers() {
    let interpreter = Interpreter::new("101 + 99");
    assert_eq!(interpreter.evaluate(), 200);
  }

  #[test]
  fn subtract_multiple_digit_numbers() {
    let interpreter = Interpreter::new("1234 - 134");
    assert_eq!(interpreter.evaluate(), 1100);
  }

  #[test]
  fn add_multiple_numbers() {
    let interpreter = Interpreter::new("1 + 2 + 3 + 4 + 5");
    assert_eq!(interpreter.evaluate(), 15);
  }

  #[test]
  fn add_and_subtract_multiple_numbers() {
    let interpreter = Interpreter::new("1 + 2 - 3 + 4 - 5");
    assert_eq!(interpreter.evaluate(), -1);
  }

  #[test]
  fn muliply_and_divide_multiple_numbers() {
    let interpreter = Interpreter::new("10 * 20 / 2 / 10");
    assert_eq!(interpreter.evaluate(), 10);
  }

  #[test]
  fn evaluate_multiterm_expression_contain_parens() {
    let interpreter = Interpreter::new("6 * (3 + 7) / 2");
    assert_eq!(interpreter.evaluate(), 30);
  }
}
