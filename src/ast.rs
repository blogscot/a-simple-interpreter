use node::{Evaluator, Node, NodeVisitor};
use parser::Parser;

pub struct Ast {
  pub root_node: Box<Node>,
}

impl Ast {
  pub fn new(text: &str) -> Self {
    let mut parser = Parser::new(&text);
    let root_node = parser.parse();
    Ast { root_node }
  }
  pub fn evaluate(&self) -> i32 {
    self.accept(&Evaluator {})
  }
}

impl Node for Ast {
  fn accept(&self, visitor: &NodeVisitor) -> i32 {
    self.root_node.accept(visitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_two_single_digit_numbers() {
    let ast = Ast::new("4 + 7");
    let result = ast.evaluate();

    assert_eq!(result, 11);
  }

  #[test]
  fn subtract_two_single_digit_numbers() {
    let ast = Ast::new("4 - 7");
    let result = ast.evaluate();

    assert_eq!(result, -3);
  }

  #[test]
  fn multiply_two_single_digit_numbers() {
    let ast = Ast::new("4 * 7");
    let result = ast.evaluate();

    assert_eq!(result, 28);
  }

  #[test]
  fn divide_two_single_digit_numbers() {
    let ast = Ast::new("10 / 3");
    let result = ast.evaluate();

    assert_eq!(result, 3);
  }

  #[test]
  fn add_multiple_digit_numbers() {
    let ast = Ast::new("101 + 99");
    let result = ast.evaluate();

    assert_eq!(result, 200);
  }

  #[test]
  fn subtract_multiple_digit_numbers() {
    let ast = Ast::new("1234 - 134");
    let result = ast.evaluate();

    assert_eq!(result, 1100);
  }

  #[test]
  fn add_multiple_numbers() {
    let ast = Ast::new("1 + 2 + 3 + 4 + 5");
    let result = ast.evaluate();

    assert_eq!(result, 15);
  }

  #[test]
  fn add_and_subtract_multiple_numbers() {
    let ast = Ast::new("1 + 2 - 3 + 4 - 5");
    let result = ast.evaluate();

    assert_eq!(result, -1);
  }

  #[test]
  fn muliply_and_divide_multiple_numbers() {
    let ast = Ast::new("10 * 20 / 2 / 10");
    let result = ast.evaluate();

    assert_eq!(result, 10);
  }

  #[test]
  fn evaluate_multiterm_expression_contain_parens() {
    let ast = Ast::new("6 * (3 + 7) / 2");
    let result = ast.evaluate();

    assert_eq!(result, 30);
  }
}
