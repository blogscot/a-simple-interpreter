use std::fmt;
use token::TokenType;
use visitor::NodeVisitor;

use mopa;

pub trait Node: mopa::Any {
  fn accept(&self, visitor: &NodeVisitor) -> i32;
}

mopafy!(Node);

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

pub struct UnaryOpNode {
  pub operator: TokenType,
  pub expr: Box<Node>,
}

impl UnaryOpNode {
  pub fn new(operator: TokenType, expr: Box<Node>) -> Self {
    UnaryOpNode { operator, expr }
  }
}

impl Node for UnaryOpNode {
  fn accept(&self, visitor: &NodeVisitor) -> i32 {
    visitor.visit_unaryop(self)
  }
}
