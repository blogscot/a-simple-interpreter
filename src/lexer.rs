use token::Token;
use token::TokenType::*;

#[derive(Clone)]
pub struct Lexer {
  text: String,
  position: usize,
  current_char: Option<char>,
}

impl Lexer {
  pub fn new(text: String) -> Self {
    Lexer {
      text: text.clone(),
      position: 0,
      current_char: Some(text.as_bytes()[0] as char),
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
        char if char.is_digit(10) => Some(Token {
          token_type: Integer(self.integer()),
        }),
        '+' => {
          self.advance();
          Some(Token { token_type: Plus })
        }
        '-' => {
          self.advance();
          Some(Token { token_type: Minus })
        }
        '*' => {
          self.advance();
          Some(Token {
            token_type: Multiply,
          })
        }
        '/' => {
          self.advance();
          Some(Token { token_type: Divide })
        }
        _ => panic!("Unknown token found!"),
      };
    }
    Some(Token { token_type: EOF })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn build_token(value: i32) -> Token {
    Token {
      token_type: Integer(value),
    }
}
  fn build_token(value: i32) -> Token {
    Token {
      token_type: Integer(value),
    }
  }

  #[test]
  fn add_two_single_digit_numbers() {
    let mut lexer = Lexer::new("4 + 7".into());
    let four = build_token(4);
    let seven = build_token(7);
    let plus = Token { token_type: Plus };

    assert_eq!(lexer.get_next_token().unwrap(), four);
    assert_eq!(lexer.get_next_token().unwrap(), plus);
    assert_eq!(lexer.get_next_token().unwrap(), seven);
  }

  #[test]
  fn multiply_two_single_digit_numbers() {
    let mut lexer = Lexer::new("4 * 7".into());
    let four = build_token(4);
    let seven = build_token(7);
    let multiply = Token {
      token_type: Multiply,
    };

    assert_eq!(lexer.get_next_token().unwrap(), four);
    assert_eq!(lexer.get_next_token().unwrap(), multiply);
    assert_eq!(lexer.get_next_token().unwrap(), seven);
  }

}
