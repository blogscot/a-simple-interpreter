use node::*;
use token::Token::*;

use std::collections::HashMap;

pub trait NodeVisitor {
  fn visit(&mut self, node: &Box<Node>) -> i32 {
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
  fn visit_num(&mut self, node: &NumNode) -> i32;
  fn visit_binop(&mut self, node: &BinOpNode) -> i32;
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> i32;
  fn visit_compound(&mut self, node: &CompoundNode) -> i32;
  fn visit_assign(&mut self, node: &AssignNode) -> i32;
  fn visit_var(&mut self, node: &VarNode) -> i32;
  fn visit_noop(&mut self, _node: &NoOpNode) -> i32 {
    0
  }
}

pub struct Evaluator {
  global_scope: HashMap<String, i32>,
}

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {
      global_scope: HashMap::new(),
    }
  }
}

impl NodeVisitor for Evaluator {
  fn visit_num(&mut self, node: &NumNode) -> i32 {
    node.value
  }
  fn visit_binop(&mut self, node: &BinOpNode) -> i32 {
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
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> i32 {
    let UnaryOpNode { operator, expr } = node;
    match operator {
      Plus => self.visit(expr),
      Minus => -self.visit(expr),
      _ => panic!("Unexpected Unary Operator found: {}", operator),
    }
  }
  fn visit_compound(&mut self, node: &CompoundNode) -> i32 {
    for child in &node.children {
      self.visit(child);
    }
    0
  }
  fn visit_assign(&mut self, node: &AssignNode) -> i32 {
    if node.identifier.is::<VarNode>() {
      let var_node: &VarNode = node.identifier.downcast_ref().unwrap();
      if let Id(name) = &var_node.identifier {
        let value = self.visit(&node.expr);
        self.global_scope.insert(name.to_string(), value);
      }
    }
    0
  }
  fn visit_var(&mut self, node: &VarNode) -> i32 {
    if let VarNode {
      identifier: Id(name),
    } = node
    {
      match self.global_scope.get(name.as_str()) {
        Some(value) => *value,
        None => panic!("Variable {} not found", name),
      }
    } else {
      panic!("Invalid identifier found {}", node.identifier);
    }
  }
}
