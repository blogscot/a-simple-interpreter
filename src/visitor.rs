use node::*;
use token::Token::*;

pub trait NodeVisitor {
  fn visit(&self, node: &Box<Node>) -> i32 {
    if node.is::<NumNode>() {
      self.visit_num(node.downcast_ref().unwrap())
    } else if node.is::<BinOpNode>() {
      self.visit_binop(node.downcast_ref().unwrap())
    } else if node.is::<UnaryOpNode>() {
      self.visit_unaryop(node.downcast_ref().unwrap())
    } else if node.is::<CompoundNode>() {
      self.visit_compound(node.downcast_ref().unwrap())
    } else if node.is::<AssignNode>() {
      self.visit_assign(node.downcast_ref().unwrap())
    } else if node.is::<VarNode>() {
      self.visit_var(node.downcast_ref().unwrap())
    } else if node.is::<NoOpNode>() {
      self.visit_noop(node.downcast_ref().unwrap())
    } else {
      panic!("Unknown node found: {}", to_string(node));
    }
  }
  fn visit_num(&self, node: &NumNode) -> i32;
  fn visit_binop(&self, node: &BinOpNode) -> i32;
  fn visit_unaryop(&self, node: &UnaryOpNode) -> i32;
  fn visit_compound(&self, node: &CompoundNode) -> i32;
  fn visit_assign(&self, node: &AssignNode) -> i32;
  fn visit_var(&self, node: &VarNode) -> i32;
  fn visit_noop(&self, _node: &NoOpNode) -> i32 {
    0
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
    match operator {
      Plus => lhs + rhs,
      Multiply => lhs * rhs,
      Minus => lhs - rhs,
      Divide => lhs / rhs,
      _ => panic!("Unknown operator found: {}", operator),
    }
  }
  fn visit_unaryop(&self, node: &UnaryOpNode) -> i32 {
    let UnaryOpNode { operator, expr } = node;
    match operator {
      Plus => self.visit(expr),
      Minus => -self.visit(expr),
      _ => panic!("Unexpected Unary Operator found: {}", operator),
    }
  }
  fn visit_compound(&self, _node: &CompoundNode) -> i32 {
    0
  }
  fn visit_assign(&self, _node: &AssignNode) -> i32 {
    0
  }
  fn visit_var(&self, node: &VarNode) -> i32 {
    if let VarNode {
      identifier: Id(_name),
    } = node
    {
      // write something here
    }
    0
  }
}
