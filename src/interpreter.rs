use evaluator::Evaluator;
use node::Node;
use number::Number;
use parser::Parser;
use visitor::NodeVisitor;

pub struct Interpreter {
  pub root_node: Box<Node>,
}

impl Interpreter {
  pub fn new(text: &str) -> Self {
    let mut parser = Parser::new(&text);
    let root_node = parser.parse();
    Interpreter { root_node }
  }
  pub fn interpret(&mut self) -> Number {
    self.accept(&mut Evaluator::new())
  }
}

impl Node for Interpreter {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> Number {
    self.root_node.accept(visitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use number::Number::Undefined;

  #[test]
  fn begin_then_end() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM empty;
    BEGIN 
    END."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  fn reserved_words_can_be_in_either_case() {
    let mut interpreter = Interpreter::new(
      r#"
    program empty;
    begin 
    end."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  #[should_panic]
  fn begin_then_end_without_period() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM empty; 
    BEGIN END"#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  fn program_contains_single_statement() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM single;
    VAR a : INTEGER;
    BEGIN 
      a := 10; 
    END."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  fn final_statement_does_not_require_semicolon() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM NoSemiColon;
    VAR a : INTEGER;
    BEGIN 
      a := 10
    END."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  #[should_panic]
  fn assignment_requires_colon() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM RequiresColon;
    VAR a : INTEGER;
    BEGIN 
      a = 10
    END."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  #[should_panic]
  fn assignment_only_has_one_equals_sign() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM EqualSign;
    VAR a : INTEGER;
    BEGIN 
      a :== 10
    END."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  fn identifiers_can_start_with_underscore() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM StartUnderscore;
    VAR _a : INTEGER;
    BEGIN 
      _a := 10
    END."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  #[should_panic]
  fn identifiers_cannot_contain_underscores() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM NoUnderscore;
    VAR an_int : INTEGER;
    BEGIN 
      an_int := 10
    END."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  fn program_contains_multiple_statements() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM multiple;
    VAR a, b, result : INTEGER;
    BEGIN 
      a := 10 * 4;
      b := -19 + 1;
      result := a DIV b - 1 
    END."#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  fn program_contains_a_compound_statement() {
    let mut interpreter = Interpreter::new(
      r#"
    Program multiple;
    Var a, b, result : INTEGER;
    Begin
      Begin
        a := 10 * 4;
        b := -19 + 1;
        result := a div b - 1
      End
    End.
    "#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  fn program_contains_a_compound_statement_with_trailing_statement() {
    let mut interpreter = Interpreter::new(
      r#"
    Program multiple;
    Var a, b, c, result : INTEGER;
    Begin
      Begin
        a := 10 * 4;
        b := -19 + 1;
        result := a / b - 1
      End;
      c := 22 / 7;
    End.
    "#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

  #[test]
  #[should_panic]
  fn program_contains_a_compound_statement_with_trailing_statement_missing_semicolon() {
    let mut interpreter = Interpreter::new(
      r#"
    Program multiple;
    Var a, b, c, result : INTEGER;
    Begin
      Begin
        a := 10 * 4;
        b := -19 + 1;
        result := a div b - 1
      End
      c := 22 DIV 3;
    End.
    "#,
    );
    assert_eq!(interpreter.interpret(), Undefined);
  }

}
