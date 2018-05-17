use node::*;
use number::Number;
use number::Number::Undefined;

pub trait NodeVisitor {
  fn visit(&mut self, node: &Box<Node>) -> Number {
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
  fn visit_program(&mut self, node: &ProgramNode) -> Number;
  fn visit_block(&mut self, node: &BlockNode) -> Number;
  fn visit_declaration(&mut self, node: &DeclarationNode) -> Number;
  fn visit_type(&mut self, node: &TypeNode) -> Number;
  fn visit_integer(&mut self, node: &IntegerNumNode) -> Number;
  fn visit_real(&mut self, node: &RealNumNode) -> Number;
  fn visit_binop(&mut self, node: &BinOpNode) -> Number;
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> Number;
  fn visit_compound(&mut self, node: &CompoundNode) -> Number;
  fn visit_assign(&mut self, node: &AssignNode) -> Number;
  fn visit_var(&mut self, node: &VarNode) -> Number;
  fn visit_noop(&mut self, _node: &NoOpNode) -> Number {
    Undefined
  }
}
