use node::*;
use number::{Number::Nil, NumberResult};

pub trait NodeVisitor {
  fn visit(&mut self, node: &Box<Node>) -> NumberResult {
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
  fn visit_program(&mut self, node: &ProgramNode) -> NumberResult;
  fn visit_block(&mut self, node: &BlockNode) -> NumberResult;
  fn visit_declaration(&mut self, node: &DeclarationNode) -> NumberResult;
  fn visit_type(&mut self, node: &TypeNode) -> NumberResult;
  fn visit_integer(&mut self, node: &IntegerNumNode) -> NumberResult;
  fn visit_real(&mut self, node: &RealNumNode) -> NumberResult;
  fn visit_binop(&mut self, node: &BinOpNode) -> NumberResult;
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> NumberResult;
  fn visit_compound(&mut self, node: &CompoundNode) -> NumberResult;
  fn visit_assign(&mut self, node: &AssignNode) -> NumberResult;
  fn visit_var(&mut self, node: &VarNode) -> NumberResult;
  fn visit_noop(&mut self, _node: &NoOpNode) -> NumberResult {
    Ok(Nil)
  }
}
