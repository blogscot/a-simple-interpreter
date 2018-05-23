use ast::node::*;
use ast::visitor::NodeVisitor;
use lexer::token::Token::Id;
use symbols::symbol::{Symbol, Symbolize};
use symbols::symbol_table::SymbolTable;
use utils::number::{Number::Nil, NumberResult};

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
  fn visit_program(&mut self, node: &ProgramNode) -> NumberResult {
    self.visit(&node.block)
  }
  fn visit_procedure(&mut self, _node: &ProcedureNode) -> NumberResult {
    Ok(Nil)
  }
  fn visit_block(&mut self, node: &BlockNode) -> NumberResult {
    for declaration in &node.declarations {
      self.visit(&declaration)?;
    }
    self.visit(&node.compound_statement)
  }
  fn visit_declaration(&mut self, node: &DeclarationNode) -> NumberResult {
    let DeclarationNode {
      var_node: VarNode { identifier },
      type_node: TypeNode { token },
    } = node;

    if let Id(name) = identifier {
      if self.symbol_table.lookup(name) != None {
        return Err(format!(
          "Found duplicate variable declaration for '{}'!",
          name
        ));
      }
      let builtin_type = &self.symbol_table.get(&token);
      let variable = Symbol::new(name, builtin_type);
      self.symbol_table.define(variable);
    }
    Ok(Nil)
  }
  fn visit_type(&mut self, _node: &TypeNode) -> NumberResult {
    Ok(Nil)
  }
  fn visit_integer(&mut self, _node: &IntegerNumNode) -> NumberResult {
    Ok(Nil)
  }
  fn visit_real(&mut self, _node: &RealNumNode) -> NumberResult {
    Ok(Nil)
  }
  fn visit_binop(&mut self, node: &BinOpNode) -> NumberResult {
    self.visit(&node.left)?;
    self.visit(&node.right)
  }
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> NumberResult {
    self.visit(&node.expr)
  }
  fn visit_compound(&mut self, node: &CompoundNode) -> NumberResult {
    for child in &node.children {
      self.visit(child)?;
    }
    Ok(Nil)
  }
  fn visit_assign(&mut self, node: &AssignNode) -> NumberResult {
    let var_node: &VarNode = node.identifier.downcast_ref().unwrap();
    if let Id(name) = &var_node.identifier {
      if self.symbol_table.lookup(&name) == None {
        return Err(format!("Undeclared variable {} found.", name));
      }
    }
    self.visit(&node.expr)
  }
  fn visit_var(&mut self, node: &VarNode) -> NumberResult {
    if let Id(name) = &node.identifier {
      if self.symbol_table.lookup(&name) == None {
        return Err(format!("Undeclared variable {} found.", name));
      }
    }
    Ok(Nil)
  }
}
