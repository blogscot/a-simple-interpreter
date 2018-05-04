use std::fmt;
use token::Token;
use visitor::NodeVisitor;

use mopa;

pub trait Node: mopa::Any {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32;
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
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32 {
    visitor.visit_num(self)
  }
}

pub struct BinOpNode {
  pub left: Box<Node>,
  pub right: Box<Node>,
  pub operator: Token,
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
  pub fn new(left: Box<Node>, right: Box<Node>, operator: Token) -> Self {
    BinOpNode {
      left,
      right,
      operator,
    }
  }
}

impl Node for BinOpNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32 {
    visitor.visit_binop(self)
  }
}

pub struct UnaryOpNode {
  pub operator: Token,
  pub expr: Box<Node>,
}

impl UnaryOpNode {
  pub fn new(operator: Token, expr: Box<Node>) -> Self {
    UnaryOpNode { operator, expr }
  }
}

impl Node for UnaryOpNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32 {
    visitor.visit_unaryop(self)
  }
}

pub struct CompoundNode {
  pub children: Vec<Box<Node>>,
}

impl CompoundNode {
  pub fn new(children: Vec<Box<Node>>) -> Self {
    CompoundNode { children }
  }
}

impl Node for CompoundNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32 {
    visitor.visit_compound(self)
  }
}

pub struct AssignNode {
  pub identifier: Box<Node>,
  pub expr: Box<Node>,
  pub operator: Token,
}

impl AssignNode {
  pub fn new(identifier: Box<Node>, expr: Box<Node>, operator: Token) -> Self {
    AssignNode {
      identifier,
      expr,
      operator,
    }
  }
}

impl Node for AssignNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32 {
    visitor.visit_assign(self)
  }
}

pub struct VarNode {
  pub identifier: Token,
}

impl VarNode {
  pub fn new(identifier: Token) -> Self {
    VarNode { identifier }
  }
}

impl Node for VarNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32 {
    visitor.visit_var(self)
  }
}

pub struct NoOpNode {}

impl Node for NoOpNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32 {
    visitor.visit_noop(self)
  }
}
