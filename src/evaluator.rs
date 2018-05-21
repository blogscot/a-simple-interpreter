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
  fn visit_declaration(&mut self, _node: &DeclarationNode) -> Number {
    Nil
  }
  fn visit_type(&mut self, _node: &TypeNode) -> Number {
    Nil
  }
  fn visit_integer(&mut self, node: &IntegerNumNode) -> Number {
    Number::from(node.value)
  }
  fn visit_real(&mut self, node: &RealNumNode) -> Number {
    Number::from(node.value)
  }
  fn visit_binop(&mut self, node: &BinOpNode) -> Number {
    let BinOpNode {
      left,
      right,
      operator,
    } = node;

    let lhs = self.visit(left);
    let rhs = self.visit(right);
    match operator {
      Plus => lhs + rhs,
      Multiply => lhs * rhs,
      Minus => lhs - rhs,
      IntegerDivision => lhs / rhs,
      RealDivision => lhs / rhs,
      _ => panic!("Unknown operator found: {}", operator),
    }
  }
  fn visit_unaryop(&mut self, node: &UnaryOpNode) -> Number {
    let UnaryOpNode { operator, expr } = node;
    match operator {
      Plus => self.visit(expr),
      Minus => -self.visit(expr),
      _ => panic!("Unexpected Unary Operator found: {}", operator),
    }
  }
  fn visit_compound(&mut self, node: &CompoundNode) -> Number {
    for child in &node.children {
      self.visit(child);
    }
    Nil
  }
  fn visit_assign(&mut self, node: &AssignNode) -> Number {
    if node.identifier.is::<VarNode>() {
      let var_node: &VarNode = node.identifier.downcast_ref().unwrap();
      if let Id(name) = &var_node.identifier {
        let value = self.visit(&node.expr);
        self
          .global_scope
          .insert(name.to_string(), value.to_string());
      }
    }
    Nil
  }
  fn visit_var(&mut self, node: &VarNode) -> Number {
    if let VarNode {
      identifier: Id(name),
    } = node
    {
      match self.global_scope.get(name.as_str()) {
        Some(value) => Number::from_str(value).unwrap(),
        None => panic!("Possible use of uninitialised variable {}.", name),
      }
    } else {
      panic!("Invalid identifier found {}", node.identifier);
    }
  }
}
