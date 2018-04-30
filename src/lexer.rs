use token::Token;
use token::TokenType;
use token::TokenType::*;

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
  fn advance(&mut self) {
    self.position += 1;
    if self.position > self.text.len() - 1 {
      self.current_char = None
    } else {
      self.current_char = Some(self.text.as_bytes()[self.position] as char)
    }
  }
  fn skip_whitespace(&mut self) {
    while self.current_char != None && self.current_char.unwrap().is_whitespace() {
      self.advance()
    }
  }
  fn integer(&mut self) -> i32 {
    let mut digits = String::new();
    while self.current_char != None && self.current_char.unwrap().is_digit(10) {
      digits.push(self.current_char.unwrap());
      self.advance();
    }
    digits.parse::<i32>().unwrap()
  }
  pub fn get_next_token(&mut self) -> Option<Token> {
    while self.current_char != None {
      return match self.current_char.unwrap() {
        char if char.is_whitespace() => {
          self.skip_whitespace();
          continue;
        }
        char if char.is_digit(10) => Some(Token::new(Integer(self.integer()))),
        '+' => {
          self.advance();
          Some(Token::new(Plus))
        }
        '-' => {
          self.advance();
          Some(Token::new(Minus))
        }
        '*' => {
          self.advance();
          Some(Token::new(Multiply))
        }
        '/' => {
          self.advance();
          Some(Token::new(Divide))
        }
        '(' => {
          self.advance();
          Some(Token::new(LParen))
        }
        ')' => {
          self.advance();
          Some(Token::new(RParen))
        }
        _ => panic!("Unknown token found!"),
      };
    }
    Some(Token::new(TokenType::EOF))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn build_token(value: i32) -> Token {
    Token::new(Integer(value))
  }

  #[test]
  fn add_two_single_digit_numbers() {
    let mut lexer = Lexer::new("4 + 7".into());

    assert_eq!(lexer.get_next_token().unwrap(), build_token(4));
    assert_eq!(lexer.get_next_token().unwrap(), Token::new(Plus));
    assert_eq!(lexer.get_next_token().unwrap(), build_token(7));
    assert_eq!(lexer.get_next_token().unwrap(), Token::new(EOF));
  }

  #[test]
  fn multiply_two_single_digit_numbers() {
    let mut lexer = Lexer::new("4 * 7".into());

    assert_eq!(lexer.get_next_token().unwrap(), build_token(4));
    assert_eq!(lexer.get_next_token().unwrap(), Token::new(Multiply));
    assert_eq!(lexer.get_next_token().unwrap(), build_token(7));
    assert_eq!(lexer.get_next_token().unwrap(), Token::new(EOF));
  }

  #[test]
  fn lex_expression_in_parens() {
    let mut lexer = Lexer::new("(4 - 7)".into());

    assert_eq!(lexer.get_next_token().unwrap(), Token::new(LParen));
    assert_eq!(lexer.get_next_token().unwrap(), build_token(4));
    assert_eq!(lexer.get_next_token().unwrap(), Token::new(Minus));
    assert_eq!(lexer.get_next_token().unwrap(), build_token(7));
    assert_eq!(lexer.get_next_token().unwrap(), Token::new(RParen));
    assert_eq!(lexer.get_next_token().unwrap(), Token::new(EOF));
  }

}
