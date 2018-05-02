use node::{BinOpNode, Node, NumNode, UnaryOpNode};
use token::Token;
use token::Token::*;

pub trait NodeVisitor {
  fn visit(&self, node: &Box<Node>) -> i32 {
    if node.is::<NumNode>() {
      self.visit_num(node.downcast_ref().unwrap())
    } else if node.is::<BinOpNode>() {
      self.visit_binop(node.downcast_ref().unwrap())
    } else if node.is::<UnaryOpNode>() {
      self.visit_unaryop(node.downcast_ref().unwrap())
    } else {
      panic!("Unknown node found!");
    }
  }
  fn visit_num(&self, node: &NumNode) -> i32;
  fn visit_binop(&self, node: &BinOpNode) -> i32;
  fn visit_unaryop(&self, node: &UnaryOpNode) -> i32;
}

pub struct Evaluator {}
impl NodeVisitor for Evaluator {
  fn visit_num(&self, node: &NumNode) -> i32 {
    node.value
  }
  fn visit_binop(&self, node: &BinOpNode) -> i32 {
    let BinOpNode {
      left,
      right,
      operator,
    } = node;

    let lhs = self.visit(left);
    let rhs = self.visit(right);
    evaluate(lhs, rhs, &operator)
  }
  fn visit_unaryop(&self, node: &UnaryOpNode) -> i32 {
    let UnaryOpNode { operator, expr } = node;
    match operator {
      Plus => self.visit(expr),
      Minus => -self.visit(expr),
      _ => panic!("Unexpected Unary Operator found: {}", operator),
    }
  }
}

pub fn evaluate(lhs: i32, rhs: i32, operator: &Token) -> i32 {
  match operator {
    Plus => lhs + rhs,
    Multiply => lhs * rhs,
    Minus => lhs - rhs,
    Divide => lhs / rhs,
    _ => panic!("Unknown operator found: {}", operator),
  }
}
