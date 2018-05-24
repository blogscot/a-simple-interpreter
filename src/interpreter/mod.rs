use ast::node::Node;
use ast::visitor::NodeVisitor;
use ast::{evaluator::Evaluator, table_builder::TableBuilder};
use parser::Parser;
use utils::number::NumberResult;

pub struct Interpreter {
  pub root_node: Box<Node>,
}

impl Interpreter {
  pub fn new(text: &str) -> Self {
    let mut parser = Parser::new(&text);
    let root_node = parser.parse();
    Interpreter { root_node }
  }
  pub fn interpret(&mut self) -> NumberResult {
    self.accept(&mut TableBuilder::new())?;
    self.accept(&mut Evaluator::new())
  }
}

impl Node for Interpreter {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    self.root_node.accept(visitor)
  }
}
