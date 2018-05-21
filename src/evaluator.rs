use std::collections::HashMap;

use node::*;
use number::Number;
use number::Number::Nil;
use std::str::FromStr;
use token::Token::*;
use visitor::NodeVisitor;

pub struct Evaluator {
  global_scope: HashMap<String, String>,
}

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {
      global_scope: HashMap::new(),
    }
  }
}

impl NodeVisitor for Evaluator {
  fn visit_program(&mut self, node: &ProgramNode) -> Result<Number, String> {
    self.visit(&node.block)
  }
  fn visit_block(&mut self, node: &BlockNode) -> Result<Number, String> {
    for declaration in &node.declarations {
      self.visit(&declaration)?;
    }
    self.visit(&node.compound_statement)
  }
  fn visit_declaration(&mut self, _node: &DeclarationNode) -> Result<Number, String> {
    Ok(Nil)
  }
  fn visit_type(&mut self, _node: &TypeNode) -> Result<Number, String> {
    Ok(Nil)
  }
  fn visit_integer(&mut self, node: &IntegerNumNode) -> Result<Number, String> {
    Ok(Number::from(node.value))
  }
  fn visit_real(&mut self, node: &RealNumNode) -> Result<Number, String> {
    Ok(Number::from(node.value))
  }
  fn visit_binop(&mut self, node: &BinOpNode) -> Result<Number, String> {
    let BinOpNode {
      left,
      right,
      operator,
    } = node;

    let lhs = self.visit(left);
    let rhs = self.visit(right);
    match operator {
      Plus => Ok(lhs? + rhs?),
      Multiply => Ok(lhs? * rhs?),
      Minus => Ok(lhs? - rhs?),
      IntegerDivision => Ok(lhs? / rhs?),
      RealDivision => Ok(lhs? / rhs?),
      _ => Err(format!("Unknown operator found: {}", operator)),
    }
  }
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> Result<Number, String> {
    let UnaryOpNode { operator, expr } = node;
    match operator {
      Plus => self.visit(expr),
      Minus => Ok(-self.visit(expr)?),
      _ => Err(format!("Unexpected Unary Operator found: {}", operator)),
    }
  }
  fn visit_compound(&mut self, node: &CompoundNode) -> Result<Number, String> {
    for child in &node.children {
      self.visit(child)?;
    }
    Ok(Nil)
  }
  fn visit_assign(&mut self, node: &AssignNode) -> Result<Number, String> {
    if node.identifier.is::<VarNode>() {
      let var_node: &VarNode = node.identifier.downcast_ref().unwrap();
      if let Id(name) = &var_node.identifier {
        let value = self.visit(&node.expr);
        self
          .global_scope
          .insert(name.to_string(), value?.to_string());
      }
    }
    Ok(Nil)
  }
  fn visit_var(&mut self, node: &VarNode) -> Result<Number, String> {
    if let VarNode {
      identifier: Id(name),
    } = node
    {
      match self.global_scope.get(name.as_str()) {
        Some(value) => Ok(Number::from_str(value).unwrap()),
        None => Err(format!("Possible use of uninitialised variable {}.", name)),
      }
    } else {
      Err(format!("Invalid identifier found {}", node.identifier))
    }
  }
}
