use evaluator::Evaluator;
use node::Node;
use number::NumberResult;
use parser::Parser;
use table_builder::TableBuilder;
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
  pub fn interpret(&mut self) -> NumberResult {
    self.accept(&mut TableBuilder::new())?;
    self.accept(&mut Evaluator::new())
  }
}

impl Node for Interpreter {
  fn accept(&mut self, visitor: &mut NodeVisitor) -> NumberResult {
    self.root_node.accept(visitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use number::Number::Nil;

  #[test]
  fn begin_then_end() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM empty;
    BEGIN 
    END."#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

  #[test]
  fn reserved_words_can_be_in_either_case() {
    let mut interpreter = Interpreter::new(
      r#"
    program empty;
    begin 
    end."#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

  #[test]
  #[should_panic]
  fn begin_then_end_without_period() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM empty; 
    BEGIN END"#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

  #[test]
  fn program_contains_integer_and_real_variables() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM single;
    VAR 
      a : INTEGER; 
      b : REAL;
    BEGIN 
      a := 10;
      b := 3.0;
    END."#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
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
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

  #[test]
  fn evalutate_addition_and_multiplication() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM addition;
    VAR
      a, b   : INTEGER;
    BEGIN
      a := 2;
      b := 10 * a + 10 * 4;
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

  #[test]
  fn evalutate_multiple_expression_statement() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM statement;
    VAR
      a, b   : INTEGER;
    BEGIN
      a := 42;
      b := 100 / a + (10 + a) - -1;
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

}
