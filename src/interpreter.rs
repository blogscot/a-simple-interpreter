use node::Node;
use parser::Parser;
use visitor::{Evaluator, NodeVisitor};

pub struct Interpreter {
  pub root_node: Box<Node>,
}

impl Interpreter {
  pub fn new(text: &str) -> Self {
    let mut parser = Parser::new(&text);
    let root_node = parser.parse();
    Interpreter { root_node }
  }
  pub fn interpret(&self) -> i32 {
    self.accept(&Evaluator {})
  }
}

impl Node for Interpreter {
  fn accept(&self, visitor: &NodeVisitor) -> i32 {
    self.root_node.accept(visitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn begin_then_end() {
    let interpreter = Interpreter::new(r#"BEGIN END."#);
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn begin_then_end_without_period() {
    let interpreter = Interpreter::new(r#"BEGIN END"#);
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn begin_then_end_with_lowercase() {
    let interpreter = Interpreter::new(r#"BEGIN ENd"#);
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_single_statement() {
    let interpreter = Interpreter::new(r#"BEGIN a := 10; END."#);
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_single_statement_without_final_semicolon() {
    let interpreter = Interpreter::new(r#"BEGIN a := 10 END."#);
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn program_contains_single_statement_without_colon() {
    let interpreter = Interpreter::new(r#"BEGIN a = 10 END."#);
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn program_contains_single_statement_with_double_equals() {
    let interpreter = Interpreter::new(r#"BEGIN a :== 10 END."#);
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_multiple_statements() {
    let interpreter = Interpreter::new(
      r#"
    BEGIN 
      a := 10 * 4;
      b := -19 + 1;
      result := a / b - 1 
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_a_compound_statement() {
    let interpreter = Interpreter::new(
      r#"
    BEGIN
      BEGIN
        a := 10 * 4;
        b := -19 + 1;
        result := a / b - 1
      END
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_a_compound_statement_with_trailing_statement() {
    let interpreter = Interpreter::new(
      r#"
    BEGIN
      BEGIN
        a := 10 * 4;
        b := -19 + 1;
        result := a / b - 1
      END;
      c := 22 / 3;
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn program_contains_a_compound_statement_with_trailing_statement_missing_semicolon() {
    let interpreter = Interpreter::new(
      r#"
    BEGIN
      BEGIN
        a := 10 * 4;
        b := -19 + 1;
        result := a / b - 1
      END
      c := 22 / 3;
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), 0);
  }

}
