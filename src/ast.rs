use node::{Evaluator, Node, NodeVisitor};
use parser::Parser;

pub struct Ast {
  pub root_node: Box<Node>,
}

impl Ast {
  pub fn new(text: &str) -> Self {
    let mut parser = Parser::new(&text);
    let root_node = parser.parse();
    Ast { root_node }
  }
  pub fn evaluate(&self) -> i32 {
    self.accept(&Evaluator {})
  }
}

impl Node for Ast {
  fn accept(&self, visitor: &NodeVisitor) -> i32 {
    self.root_node.accept(visitor)
  }
}
