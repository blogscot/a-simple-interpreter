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
  pub fn interpret(&mut self) -> i32 {
    self.accept(&mut Evaluator::new())
  }
}

impl Node for Interpreter {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> i32 {
    self.root_node.accept(visitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn begin_then_end() {
    let mut interpreter = Interpreter::new("BEGIN END.");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn reserved_words_can_be_in_either_case() {
    let mut interpreter = Interpreter::new("begin end.");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn begin_then_end_without_period() {
    let mut interpreter = Interpreter::new("BEGIN END");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_single_statement() {
    let mut interpreter = Interpreter::new("BEGIN a := 10; END.");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_single_statement_without_final_semicolon() {
    let mut interpreter = Interpreter::new("BEGIN a := 10 END.");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn program_contains_single_statement_without_colon() {
    let mut interpreter = Interpreter::new("BEGIN a = 10 END.");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn idenifiers_can_start_with_underscore() {
    let mut interpreter = Interpreter::new("BEGIN _a := 10 END.");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn idenifiers_cannot_contain_underscores() {
    let mut interpreter = Interpreter::new("BEGIN a_num := 10 END.");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn program_contains_single_statement_with_double_equals() {
    let mut interpreter = Interpreter::new("BEGIN a :== 10 END.");
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_multiple_statements() {
    let mut interpreter = Interpreter::new(
      r#"
    BEGIN 
      a := 10 * 4;
      b := -19 + 1;
      result := a DIV b - 1 
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_a_compound_statement() {
    let mut interpreter = Interpreter::new(
      r#"
    begin
      begin
        a := 10 * 4;
        b := -19 + 1;
        result := a div b - 1
      end
    end.
    "#,
    );
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  fn program_contains_a_compound_statement_with_trailing_statement() {
    let mut interpreter = Interpreter::new(
      r#"
    BEGIN
      BEGIN
        a := 10 * 4;
        b := -19 + 1;
        result := a DIV b - 1
      END;
      c := 22 DIV 3;
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), 0);
  }

  #[test]
  #[should_panic]
  fn program_contains_a_compound_statement_with_trailing_statement_missing_semicolon() {
    let mut interpreter = Interpreter::new(
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
