extern crate basic_interpreter;

#[cfg(test)]
mod tests {
  use basic_interpreter::{interpreter::*, utils::number::Number::Nil};

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
  fn should_panic_begin_then_end_without_period() {
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
  fn should_panic_assignment_requires_colon() {
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
  fn should_panic_assignment_only_has_one_equals_sign() {
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
  fn should_panic_identifiers_cannot_contain_underscores() {
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
  fn should_panic_compound_statement_with_trailing_statement_missing_semicolon() {
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

  #[test]
  fn parse_procedure() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM proc;
    VAR
      a : INTEGER;

      PROCEDURE P1;
      VAR
        a : REAL;
        k : INTEGER;
      BEGIN
      END;
      
    BEGIN
      a := 10;
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

  #[test]
  fn parse_nested_procedure() {
    let mut interpreter = Interpreter::new(
      r#"
    PROGRAM proc;
    VAR
      a : INTEGER;

      PROCEDURE P1;
      VAR
        a : REAL;
        k : INTEGER;

        PROCEDURE P2;
        VAR
          a, z : INTEGER;
        BEGIN {P2}
          z := 777;
        END;  {P2}

      BEGIN {P1}
      END; {P1}

    BEGIN
      a := 10;
    END.
    "#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

  #[test]
  fn interpret_formal_parameter_ints() {
    let mut interpreter = Interpreter::new(
      r#"
    program Main;
      var x, y: real;

      procedure Alpha(a, b : integer);
          var y : integer;
      begin
          x := a + x + y;
      end;

    begin

    end.
    "#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

  #[test]
  fn interpret_multiple_formal_parameters() {
    let mut interpreter = Interpreter::new(
      r#"
    program Main;
      var x, y: real;

      procedure Alpha(a, b : integer; c : real);
          var y : integer;
      begin
          x := a + x + y;
      end;

    begin

    end.
    "#,
    );
    assert_eq!(interpreter.interpret(), Ok(Nil));
  }

}
