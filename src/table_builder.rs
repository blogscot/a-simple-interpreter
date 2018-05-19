use node::*;
use number::Number;
use number::Number::Nil;
use symbol::Symbol;
use symbol::Symbolize;
use symbol_table::SymbolTable;
use token::Token::Id;
use visitor::NodeVisitor;

pub struct TableBuilder {
  symbol_table: SymbolTable,
}

impl TableBuilder {
  pub fn new() -> Self {
    TableBuilder {
      symbol_table: SymbolTable::new(),
    }
  }
}

impl NodeVisitor for TableBuilder {
  fn visit_program(&mut self, node: &ProgramNode) -> Number {
    self.visit(&node.block);
    Nil
  }
  fn visit_block(&mut self, node: &BlockNode) -> Number {
    for declaration in &node.declarations {
      self.visit(&declaration);
    }
    self.visit(&node.compound_statement)
  }
  fn visit_declaration(&mut self, node: &DeclarationNode) -> Number {
    let DeclarationNode {
      var_node: VarNode { identifier },
      type_node: TypeNode { token },
    } = node;

    if let Id(name) = identifier {
      let ref builtin_type = self.symbol_table.get(&token);
      let variable = Symbol::new(name, builtin_type);
      self.symbol_table.define(variable);
    }
    Nil
  }
  fn visit_type(&mut self, _node: &TypeNode) -> Number {
    Nil
  }
  fn visit_integer(&mut self, _node: &IntegerNumNode) -> Number {
    Nil
  }
  fn visit_real(&mut self, _node: &RealNumNode) -> Number {
    Nil
  }
  fn visit_binop(&mut self, node: &BinOpNode) -> Number {
    self.visit(&node.left);
    self.visit(&node.right);
    Nil
  }
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> Number {
    self.visit(&node.expr)
  }
  fn visit_compound(&mut self, node: &CompoundNode) -> Number {
    for child in &node.children {
      self.visit(child);
    }
    Nil
  }
  fn visit_assign(&mut self, _node: &AssignNode) -> Number {
    Nil
  }
  fn visit_var(&mut self, _node: &VarNode) -> Number {
    Nil
  }
}
