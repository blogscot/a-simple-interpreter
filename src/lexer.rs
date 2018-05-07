use token::Token;
use token::Token::*;

use std::collections::HashMap;

lazy_static! {
  static ref RESERVED_WORDS: HashMap<&'static str, Token> = {
    let mut reserved_words = HashMap::new();
    reserved_words.insert("PROGRAM", Program("".into()));
    reserved_words.insert("VAR", Program("".into()));
    reserved_words.insert("INTEGER", Integer);
    reserved_words.insert("REAL", Real);
    reserved_words.insert("BEGIN", Begin);
    reserved_words.insert("END", End);
    reserved_words.insert("DIV", IntegerDivision);
    reserved_words
  };
}

#[derive(Clone)]
pub struct Lexer {
  text: String,
  position: usize,
  current_char: Option<char>,
}

impl Lexer {
  pub fn new(text: &str) -> Self {
    let chars: Vec<char> = text.chars().collect();
    Lexer {
      text: text.to_string(),
      position: 0,
      current_char: Some(chars[0]),
    }
  }
  /// Returns an option to the character following
  /// the current token.
  pub fn peek(&self) -> Option<char> {
    let position = self.position + 1;
    if position > self.text.len() - 1 {
      None
    } else {
      Some(self.text.as_bytes()[position] as char)
    }
  }
  /// Advances the lexer position within the input text,
  /// setting the `current_char` to value found at that
  /// location.
  fn advance(&mut self) {
    self.position += 1;
    if self.position > self.text.len() - 1 {
      self.current_char = None
    } else {
      self.current_char = Some(self.text.as_bytes()[self.position] as char)
    }
  }
  fn skip_comment(&mut self) {
    while self.current_char != Some('}') {
      self.advance()
    }
    self.advance()
  }
  fn skip_whitespace(&mut self) {
    while self.current_char != None && self.current_char.unwrap().is_whitespace() {
      self.advance()
    }
  }
  /// Handles identifiers and reserved keywords
  fn id(&mut self) -> Option<Token> {
    let mut result = String::new();
    while self.current_char != None && self.current_char.unwrap().is_alphanumeric() {
      result.push(self.current_char.unwrap());
      self.advance();
    }
    let uppercase_result = result.to_uppercase();
    if RESERVED_WORDS.contains_key(uppercase_result.as_str()) {
      RESERVED_WORDS.get(uppercase_result.as_str()).cloned()
    } else {
      Some(Id(result))
    }
  }

  fn number(&mut self) -> Option<Token> {
    let mut digits = String::new();
    while self.current_char != None && self.current_char.unwrap().is_digit(10) {
      digits.push(self.current_char.unwrap());
      self.advance();

      if self.current_char == Some('.') {
        digits.push('.');
        self.advance();

        while self.current_char != None && self.current_char.unwrap().is_digit(10) {
          digits.push(self.current_char.unwrap());
          self.advance();
        }
        return Some(RealConst(digits.parse::<f32>().unwrap()));
      }
    }
    Some(IntegerConst(digits.parse::<i32>().unwrap()))
  }
  pub fn get_next_token(&mut self) -> Option<Token> {
    while self.current_char != None {
      return match self.current_char.unwrap() {
        char if char.is_whitespace() => {
          self.skip_whitespace();
          continue;
        }
        '{' => {
          self.advance();
          self.skip_comment();
          continue;
        }
        char if char.is_digit(10) => self.number(),
        '+' => {
          self.advance();
          Some(Plus)
        }
        char if char.is_alphanumeric() => self.id(),
        '_' if self.peek().unwrap().is_alphanumeric() => {
          self.advance();
          self.id()
        }
        ':' if self.peek() == Some('=') => {
          self.advance();
          self.advance();
          Some(Assign)
        }
        ';' => {
          self.advance();
          Some(Semi)
        }
        '.' => {
          self.advance();
          Some(Period)
        }
        '-' => {
          self.advance();
          Some(Minus)
        }
        '*' => {
          self.advance();
          Some(Multiply)
        }
        '(' => {
          self.advance();
          Some(LParen)
        }
        ')' => {
          self.advance();
          Some(RParen)
        }
        unknown => panic!("Unknown token found: {}", unknown),
      };
    }
    Some(Token::EOF)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_two_single_digit_numbers() {
    let mut lexer = Lexer::new("4 + 7".into());

    assert_eq!(lexer.get_next_token().unwrap(), IntegerConst(4));
    assert_eq!(lexer.get_next_token().unwrap(), Plus);
    assert_eq!(lexer.get_next_token().unwrap(), IntegerConst(7));
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

  #[test]
  fn multiply_two_single_digit_numbers() {
    let mut lexer = Lexer::new("4 * 7".into());

    assert_eq!(lexer.get_next_token().unwrap(), IntegerConst(4));
    assert_eq!(lexer.get_next_token().unwrap(), Multiply);
    assert_eq!(lexer.get_next_token().unwrap(), IntegerConst(7));
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

  #[test]
  fn multiply_two_real_numbers() {
    let mut lexer = Lexer::new("4.125 * 3.3333".into());

    assert_eq!(lexer.get_next_token().unwrap(), RealConst(4.125));
    assert_eq!(lexer.get_next_token().unwrap(), Multiply);
    assert_eq!(lexer.get_next_token().unwrap(), RealConst(3.3333));
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

  #[test]
  fn lex_expression_in_parens() {
    let mut lexer = Lexer::new("(4 - 7)".into());

    assert_eq!(lexer.get_next_token().unwrap(), LParen);
    assert_eq!(lexer.get_next_token().unwrap(), IntegerConst(4));
    assert_eq!(lexer.get_next_token().unwrap(), Minus);
    assert_eq!(lexer.get_next_token().unwrap(), IntegerConst(7));
    assert_eq!(lexer.get_next_token().unwrap(), RParen);
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

  #[test]
  fn lex_reserved_keywords() {
    let mut lexer = Lexer::new("BEGIN END.".into());

    assert_eq!(lexer.get_next_token().unwrap(), Begin);
    assert_eq!(lexer.get_next_token().unwrap(), End);
    assert_eq!(lexer.get_next_token().unwrap(), Period);
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

  #[test]
  fn lex_assignment() {
    let mut lexer = Lexer::new("a := 10;".into());

    assert_eq!(lexer.get_next_token().unwrap(), Id("a".to_string()));
    assert_eq!(lexer.get_next_token().unwrap(), Assign);
    assert_eq!(lexer.get_next_token().unwrap(), IntegerConst(10));
    assert_eq!(lexer.get_next_token().unwrap(), Semi);
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

  #[test]
  fn lex_comment() {
    let mut lexer = Lexer::new(r#"{ This is how you write a comment }"#.into());
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }
}
