use ast::node::*;
use ast::visitor::NodeVisitor;
use lexer::token::Token::Id;
use symbols::symbol::{BuiltIn, Symbol::*, SymbolTable};
use utils::number::{Number::Nil, NumberResult};

#[derive(Clone)]
pub struct TableBuilder {
  global_scope: SymbolTable,
  current_scope: SymbolTable,
}

impl TableBuilder {
  pub fn new() -> Self {
    TableBuilder {
      global_scope: Default::default(),
      current_scope: Default::default(),
    }
  }
}

impl NodeVisitor for TableBuilder {
  fn visit_program(&mut self, node: &ProgramNode) -> NumberResult {
    let global_scope = SymbolTable::new("Global Scope", 1);
    self.global_scope = global_scope.clone();
    self.current_scope = global_scope;

    let result = self.visit(&node.block);

    println!("{}", self.global_scope);
    result
  }
  fn visit_procedure(&mut self, node: &ProcedureNode) -> NumberResult {
    let proc_name = node.proc_name.to_string();
    let procedure_scope = SymbolTable::new(&proc_name, 2);
    self.current_scope = procedure_scope;

    let params: Vec<(String, BuiltIn)> = node
      .params
      .iter()
      .map(|boxed_node| boxed_node.downcast_ref().unwrap())
      .map(
        |ParameterNode {
           var_node: VarNode { identifier },
           type_node: TypeNode { token },
         }| {
          self.current_scope.insert(VarSymbol(
            identifier.to_string(),
            BuiltIn::new(token.clone()),
          ));
          (identifier.to_string(), BuiltIn::new(token.clone()))
        },
      )
      .collect();
    let _procedure_symbol = ProcedureSymbol(proc_name, params);
    println!("{}", self.current_scope);

    self.visit(&node.block)
  }
  fn visit_parameter(&mut self, _node: &ParameterNode) -> NumberResult {
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
      if self.global_scope.lookup(name) != None {
        return Err(format!(
          "Found duplicate variable declaration for '{}'!",
          name
        ));
      }
      if let BuiltInSymbol(builtin) = self.global_scope.get(&token) {
        let variable = VarSymbol(name.to_string(), builtin);
        self.global_scope.insert(variable);
      } else {
        panic!("Invalid builtin type {}", token);
      }
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
      if self.global_scope.lookup(&name) == None {
        return Err(format!("Undeclared variable {} found.", name));
      }
    }
    self.visit(&node.expr)
  }
  fn visit_var(&mut self, node: &VarNode) -> NumberResult {
    if let Id(name) = &node.identifier {
      if self.global_scope.lookup(&name) == None {
        return Err(format!("Undeclared variable {} found.", name));
      }
    }
    Ok(Nil)
  }
}
