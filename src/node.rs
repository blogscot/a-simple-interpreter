use std::fmt;
use token::TokenType;
use token::TokenType::*;

use mopa;

pub trait Node: mopa::Any {
  fn accept(&self, visitor: &NodeVisitor) -> i32;
}

mopafy!(Node);

pub trait NodeVisitor {
  fn visit(&self, node: &Box<Node>) -> i32 {
    if node.is::<NumNode>() {
      self.visit_num(node.downcast_ref().unwrap())
    } else if node.is::<BinOpNode>() {
      self.visit_binop(node.downcast_ref().unwrap())
    } else {
      panic!("Unknown node found!");
    }
  }
  fn visit_num(&self, node: &NumNode) -> i32;
  fn visit_binop(&self, node: &BinOpNode) -> i32;
}

#[derive(Debug)]
pub struct NumNode {
  pub value: i32,
}

impl NumNode {
  pub fn new(value: i32) -> Self {
    NumNode { value }
  }
}

impl Node for NumNode {
  fn accept(&self, visitor: &NodeVisitor) -> i32 {
    visitor.visit_num(self)
  }
}

pub struct BinOpNode {
  pub left: Box<Node>,
  pub right: Box<Node>,
  pub operator: TokenType,
}

pub fn to_string(node: &Box<Node>) -> String {
  if node.is::<NumNode>() {
    node.downcast_ref::<NumNode>().unwrap().value.to_string()
  } else {
    let BinOpNode {
      left,
      right,
      operator,
    } = node.downcast_ref::<BinOpNode>().unwrap();
    format!(
      "BinOpNode({} {} {}) ",
      to_string(left),
      to_string(right),
      operator
    )
  }
}

impl fmt::Display for BinOpNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let left = to_string(&self.left);
    let right = to_string(&self.right);
    write!(f, "BinOpNode({} {} {}) ", left, right, self.operator)
  }
}

impl BinOpNode {
  pub fn new(left: Box<Node>, right: Box<Node>, operator: TokenType) -> Self {
    BinOpNode {
      left,
      right,
      operator,
    }
  }
}

impl Node for BinOpNode {
  fn accept(&self, visitor: &NodeVisitor) -> i32 {
    visitor.visit_binop(self)
  }
}

pub fn evaluate(lhs: i32, rhs: i32, operator: TokenType) -> i32 {
  match operator {
    Plus => lhs + rhs,
    Multiply => lhs * rhs,
    Minus => lhs - rhs,
    Divide => lhs / rhs,
    _ => panic!("Unknown operator found: {}", operator),
  }
}

#[allow(dead_code)]
struct Printer {}
impl NodeVisitor for Printer {
  fn visit_num(&self, node: &NumNode) -> i32 {
    node.value
  }
  fn visit_binop(&self, node: &BinOpNode) -> i32 {
    match node {
      BinOpNode {
        left,
        right,
        operator,
      } => {
        let lhs = self.visit(left);
        let rhs = self.visit(right);
        println!("{} {} {}", lhs, operator, rhs);
        evaluate(lhs, rhs, operator.clone())
      }
    }
  }
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
    evaluate(lhs, rhs, operator.clone())
  }
}
