use node::*;
use number::Number;
use number::Number::Nil;

pub trait NodeVisitor {
  fn visit(&mut self, node: &Box<Node>) -> Result<Number, String> {
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
  fn visit_program(&mut self, node: &ProgramNode) -> Result<Number, String>;
  fn visit_block(&mut self, node: &BlockNode) -> Result<Number, String>;
  fn visit_declaration(&mut self, node: &DeclarationNode) -> Result<Number, String>;
  fn visit_type(&mut self, node: &TypeNode) -> Result<Number, String>;
  fn visit_integer(&mut self, node: &IntegerNumNode) -> Result<Number, String>;
  fn visit_real(&mut self, node: &RealNumNode) -> Result<Number, String>;
  fn visit_binop(&mut self, node: &BinOpNode) -> Result<Number, String>;
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> Result<Number, String>;
  fn visit_compound(&mut self, node: &CompoundNode) -> Result<Number, String>;
  fn visit_assign(&mut self, node: &AssignNode) -> Result<Number, String>;
  fn visit_var(&mut self, node: &VarNode) -> Result<Number, String>;
  fn visit_noop(&mut self, _node: &NoOpNode) -> Result<Number, String> {
    Ok(Nil)
  }
}
