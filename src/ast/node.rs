use ast::visitor::NodeVisitor;
use lexer::token::Token;
use std::fmt;
use utils::number::NumberResult;

use mopa;

pub trait Node: mopa::Any {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult;
}

mopafy!(Node);

#[derive(Debug)]
pub struct IntegerNumNode {
  pub value: i32,
}

impl IntegerNumNode {
  pub fn new(value: i32) -> Self {
    IntegerNumNode { value }
  }
}

impl Node for IntegerNumNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_integer(self)
  }
}

#[derive(Debug)]
pub struct RealNumNode {
  pub value: f32,
}

impl RealNumNode {
  pub fn new(value: f32) -> Self {
    RealNumNode { value }
  }
}

impl Node for RealNumNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_real(self)
  }
}

pub struct BinOpNode {
  pub left: Box<Node>,
  pub right: Box<Node>,
  pub operator: Token,
}

pub fn to_string(node: &Box<Node>) -> String {
  if node.is::<IntegerNumNode>() {
    node
      .downcast_ref::<IntegerNumNode>()
      .unwrap()
      .value
      .to_string()
  } else if node.is::<RealNumNode>() {
    node
      .downcast_ref::<RealNumNode>()
      .unwrap()
      .value
      .to_string()
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
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
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
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
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
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
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
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
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
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_var(self)
  }
}

pub struct NoOpNode {}

impl Node for NoOpNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_noop(self)
  }
}

pub struct ProgramNode {
  pub identifier: Token,
  pub block: Box<Node>,
}

impl ProgramNode {
  pub fn new(identifier: Token, block: Box<Node>) -> Self {
    ProgramNode { identifier, block }
  }
}

impl Node for ProgramNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_program(self)
  }
}

pub struct ProcedureNode {
  pub proc_name: Token,
  pub block: Box<Node>,
}

impl ProcedureNode {
  pub fn new(proc_name: Token, block: Box<Node>) -> Self {
    ProcedureNode { proc_name, block }
  }
}

impl Node for ProcedureNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_procedure(self)
  }
}

pub struct BlockNode {
  pub declarations: Vec<Box<Node>>,
  pub compound_statement: Box<Node>,
}

impl BlockNode {
  pub fn new(declarations: Vec<Box<Node>>, compound_statement: Box<Node>) -> Self {
    BlockNode {
      declarations,
      compound_statement,
    }
  }
}

impl Node for BlockNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_block(self)
  }
}

pub struct DeclarationNode {
  pub var_node: VarNode,
  pub type_node: TypeNode,
}

impl DeclarationNode {
  pub fn new(var_node: VarNode, type_node: TypeNode) -> Self {
    DeclarationNode {
      var_node,
      type_node,
    }
  }
}

impl Node for DeclarationNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_declaration(self)
  }
}

#[derive(Clone)]
pub struct TypeNode {
  pub token: Token,
}

impl TypeNode {
  pub fn new(token: Token) -> Self {
    TypeNode { token }
  }
}

impl Node for TypeNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    visitor.visit_type(self)
  }
}
