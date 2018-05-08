use node::*;
use token::Token::*;

use std::collections::HashMap;

pub trait NodeVisitor {
  fn visit(&mut self, node: &Box<Node>) -> i32 {
    if node.is::<ProgramNode>() {
      self.visit_program(node.downcast_ref().unwrap())
    } else if node.is::<BlockNode>() {
      self.visit_block(node.downcast_ref().unwrap())
    } else if node.is::<DeclarationNode>() {
      self.visit_declaration(node.downcast_ref().unwrap())
    } else if node.is::<TypeNode>() {
      self.visit_type(node.downcast_ref().unwrap())
    } else if node.is::<IntegerNumNode>() {
      self.visit_integer(node.downcast_ref().unwrap())
    } else if node.is::<RealNumNode>() {
      self.visit_real(node.downcast_ref().unwrap())
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
  fn visit_program(&mut self, node: &ProgramNode) -> i32;
  fn visit_block(&mut self, node: &BlockNode) -> i32;
  fn visit_declaration(&mut self, node: &DeclarationNode) -> i32;
  fn visit_type(&mut self, node: &TypeNode) -> i32;
  fn visit_integer(&mut self, node: &IntegerNumNode) -> i32;
  fn visit_real(&mut self, node: &RealNumNode) -> i32;
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
  fn visit_program(&mut self, node: &ProgramNode) -> i32 {
    self.visit(&node.block);
    0
  }
  fn visit_block(&mut self, node: &BlockNode) -> i32 {
    for declaration in &node.declarations {
      self.visit(&declaration);
    }
    self.visit(&node.compound_statement)
  }
  fn visit_declaration(&mut self, _node: &DeclarationNode) -> i32 {
    0
  }
  fn visit_type(&mut self, _node: &TypeNode) -> i32 {
    0
  }
  fn visit_integer(&mut self, node: &IntegerNumNode) -> i32 {
    node.value
  }
  fn visit_real(&mut self, node: &RealNumNode) -> i32 {
    node.value as i32
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
      IntegerDivision => lhs / rhs,
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
