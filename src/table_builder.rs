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
  fn visit_program(&mut self, node: &ProgramNode) -> Result<Number, String> {
    self.visit(&node.block)
  }
  fn visit_block(&mut self, node: &BlockNode) -> Result<Number, String> {
    for declaration in &node.declarations {
      self.visit(&declaration)?;
    }
    self.visit(&node.compound_statement)
  }
  fn visit_declaration(&mut self, node: &DeclarationNode) -> Result<Number, String> {
    let DeclarationNode {
      var_node: VarNode { identifier },
      type_node: TypeNode { token },
    } = node;

    if let Id(name) = identifier {
      let builtin_type = &self.symbol_table.get(&token);
      let variable = Symbol::new(name, builtin_type);
      self.symbol_table.define(variable);
    }
    Ok(Nil)
  }
  fn visit_type(&mut self, _node: &TypeNode) -> Result<Number, String> {
    Ok(Nil)
  }
  fn visit_integer(&mut self, _node: &IntegerNumNode) -> Result<Number, String> {
    Ok(Nil)
  }
  fn visit_real(&mut self, _node: &RealNumNode) -> Result<Number, String> {
    Ok(Nil)
  }
  fn visit_binop(&mut self, node: &BinOpNode) -> Result<Number, String> {
    self.visit(&node.left)?;
    self.visit(&node.right)
  }
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> Result<Number, String> {
    self.visit(&node.expr)
  }
  fn visit_compound(&mut self, node: &CompoundNode) -> Result<Number, String> {
    for child in &node.children {
      self.visit(child)?;
    }
    Ok(Nil)
  }
  fn visit_assign(&mut self, node: &AssignNode) -> Result<Number, String> {
    let var_node: &VarNode = node.identifier.downcast_ref().unwrap();
    if let Id(name) = &var_node.identifier {
      if self.symbol_table.lookup(&name) == None {
        return Err(format!("Undeclared variable {} found.", name));
      }
    }
    self.visit(&node.expr)
  }
  fn visit_var(&mut self, node: &VarNode) -> Result<Number, String> {
    if let Id(name) = &node.identifier {
      if self.symbol_table.lookup(&name) == None {
        return Err(format!("Undeclared variable {} found.", name));
      }
    }
    Ok(Nil)
  }
}
