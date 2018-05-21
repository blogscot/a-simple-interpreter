use std::collections::HashMap;

use node::*;
use number::{Number, Number::Nil, NumberResult};
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
  fn visit_program(&mut self, node: &ProgramNode) -> NumberResult {
    self.visit(&node.block)
  }
  fn visit_block(&mut self, node: &BlockNode) -> NumberResult {
    for declaration in &node.declarations {
      self.visit(&declaration)?;
    }
    self.visit(&node.compound_statement)
  }
  fn visit_declaration(&mut self, _node: &DeclarationNode) -> NumberResult {
    Ok(Nil)
  }
  fn visit_type(&mut self, _node: &TypeNode) -> NumberResult {
    Ok(Nil)
  }
  fn visit_integer(&mut self, node: &IntegerNumNode) -> NumberResult {
    Ok(Number::from(node.value))
  }
  fn visit_real(&mut self, node: &RealNumNode) -> NumberResult {
    Ok(Number::from(node.value))
  }
  fn visit_binop(&mut self, node: &BinOpNode) -> NumberResult {
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
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> NumberResult {
    let UnaryOpNode { operator, expr } = node;
    match operator {
      Plus => self.visit(expr),
      Minus => Ok(-self.visit(expr)?),
      _ => Err(format!("Unexpected Unary Operator found: {}", operator)),
    }
  }
  fn visit_compound(&mut self, node: &CompoundNode) -> NumberResult {
    for child in &node.children {
      self.visit(child)?;
    }
    Ok(Nil)
  }
  fn visit_assign(&mut self, node: &AssignNode) -> NumberResult {
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
  fn visit_var(&mut self, node: &VarNode) -> NumberResult {
    if let VarNode {
      identifier: Id(name),
    } = node
    {
      match self.global_scope.get(name.as_str()) {
        Some(value) => Ok(Number::from_str(value).unwrap()),
        None => Err(format!("Possible use of uninitialised variable: {}.", name)),
      }
    } else {
      Err(format!("Invalid identifier found {}", node.identifier))
    }
  }
}
