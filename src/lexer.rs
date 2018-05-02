use token::Token;
use token::Token::*;

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
        char if char.is_digit(10) => Some(Integer(self.integer())),
        '+' => {
          self.advance();
          Some(Plus)
        }
        '-' => {
          self.advance();
          Some(Minus)
        }
        '*' => {
          self.advance();
          Some(Multiply)
        }
        '/' => {
          self.advance();
          Some(Divide)
        }
        '(' => {
          self.advance();
          Some(LParen)
        }
        ')' => {
          self.advance();
          Some(RParen)
        }
        _ => panic!("Unknown token found!"),
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

    assert_eq!(lexer.get_next_token().unwrap(), Integer(4));
    assert_eq!(lexer.get_next_token().unwrap(), Plus);
    assert_eq!(lexer.get_next_token().unwrap(), Integer(7));
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

  #[test]
  fn multiply_two_single_digit_numbers() {
    let mut lexer = Lexer::new("4 * 7".into());

    assert_eq!(lexer.get_next_token().unwrap(), Integer(4));
    assert_eq!(lexer.get_next_token().unwrap(), Multiply);
    assert_eq!(lexer.get_next_token().unwrap(), Integer(7));
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

  #[test]
  fn lex_expression_in_parens() {
    let mut lexer = Lexer::new("(4 - 7)".into());

    assert_eq!(lexer.get_next_token().unwrap(), LParen);
    assert_eq!(lexer.get_next_token().unwrap(), Integer(4));
    assert_eq!(lexer.get_next_token().unwrap(), Minus);
    assert_eq!(lexer.get_next_token().unwrap(), Integer(7));
    assert_eq!(lexer.get_next_token().unwrap(), RParen);
    assert_eq!(lexer.get_next_token().unwrap(), EOF);
  }

}
