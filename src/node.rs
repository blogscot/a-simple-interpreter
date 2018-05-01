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

#[allow(dead_code)]
pub struct Ast {
  pub root_node: Box<Node>,
}

impl Ast {
  #[allow(dead_code)]
  pub fn new(root_node: Box<Node>) -> Self {
    Ast { root_node }
  }
  #[allow(dead_code)]
  fn evaluate(&self) -> i32 {
    self.accept(&Evaluator {})
  }
}

impl Node for Ast {
  fn accept(&self, visitor: &NodeVisitor) -> i32 {
    self.root_node.accept(visitor)
  }
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn evaluate_an_integer() {
    let value = 123;
    let node = Box::new(NumNode::new(value));

    let ast = Ast::new(node);
    assert_eq!(value, ast.accept(&Evaluator {}));
  }

  #[test]
  fn add_two_integers() {
    let node1 = Box::new(NumNode::new(10));
    let node2 = Box::new(NumNode::new(-4));
    let plus = Plus;

    let root = Box::new(BinOpNode::new(node1, node2, plus));
    let ast = Ast::new(root);
    assert_eq!(6, ast.accept(&Evaluator {}));
  }

  #[test]
  fn multiply_two_integers() {
    let node1 = Box::new(NumNode::new(10));
    let node2 = Box::new(NumNode::new(-4));
    let times = Multiply;

    let root = Box::new(BinOpNode::new(node1, node2, times));
    let ast = Ast::new(root);
    assert_eq!(-40, ast.accept(&Evaluator {}));
  }

  #[test]
  fn subtract_two_integers() {
    let node1 = Box::new(NumNode::new(10));
    let node2 = Box::new(NumNode::new(-4));
    let subtract = Minus;

    let root = Box::new(BinOpNode::new(node1, node2, subtract));
    let ast = Ast::new(root);
    assert_eq!(14, ast.accept(&Evaluator {}));
  }

  #[test]
  fn divide_two_integers() {
    let node1 = Box::new(NumNode::new(10));
    let node2 = Box::new(NumNode::new(-4));
    let divide = Divide;

    let root = Box::new(BinOpNode::new(node1, node2, divide));
    let ast = Ast::new(root);
    assert_eq!(-2, ast.accept(&Evaluator {}));
  }

  #[test]
  fn evaluate_multiple_expressions() {
    // "14 - (3 + 9 * 7) / 7 = 10"
    //            root
    //             /
    //          -      7
    //      14     *
    //          +      7
    //      3      9
    //
    let node1 = Box::new(NumNode::new(3));
    let node2 = Box::new(NumNode::new(9));
    let node3 = Box::new(NumNode::new(7));
    let node4 = Box::new(NumNode::new(14));
    let node5 = Box::new(NumNode::new(7));
    let plus = Plus;
    let subtract = Minus;
    let times = Multiply;
    let divide = Divide;
    let bin_op1 = Box::new(BinOpNode::new(node1, node2, plus));
    let bin_op2 = Box::new(BinOpNode::new(bin_op1, node3, times));
    let bin_op3 = Box::new(BinOpNode::new(node4, bin_op2, subtract));
    let root = Box::new(BinOpNode::new(bin_op3, node5, divide));

    let ast = Ast::new(root);
    assert_eq!(-10, ast.accept(&Evaluator {}));
  }

}
