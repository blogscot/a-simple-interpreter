use token::Token;
use token::Token::*;

use lexer::Lexer;
use node::{BinOpNode, Node, NumNode, UnaryOpNode};

#[derive(Clone)]
pub struct Parser {
  lexer: Lexer,
  current_token: Option<Token>,
}

impl Parser {
  pub fn new(text: &str) -> Self {
    let mut lexer = Lexer::new(&text);
    let current_token = lexer.get_next_token();

    Parser {
      lexer,
      current_token,
    }
  }
  fn get_current_token(&self) -> Token {
    self.current_token.clone().unwrap()
  }
  ///
  /// Verifies the token type matches the current token type.
  /// If valid the next token is saved.
  ///
  fn consume(&mut self, token_type: &Token) {
    let current_token = self.get_current_token();

    if current_token == *token_type {
      self.current_token = self.lexer.get_next_token();
    } else {
      panic!(format!("consume: token error: {}", current_token));
    }
  }
  fn factor(&mut self) -> Box<Node> {
    // factor : <PLUS | Minus) Factor | Integer | LParen expr RParen
    let current_token = self.get_current_token();

    if current_token == Plus || current_token == Minus {
      self.consume(&current_token);
      let node = UnaryOpNode::new(current_token, self.factor());
      Box::new(node)
    } else if let Integer(value) = current_token {
      self.consume(&current_token);
      Box::new(NumNode::new(value))
    } else if let LParen = current_token {
      self.consume(&current_token);
      let node = self.expr();
      self.consume(&Token::RParen);
      node
    } else {
      panic!(format!("Invalid factor: {}", current_token));
    }
  }
  fn term(&mut self) -> Box<Node> {
    // factor ((Multiply | Divide) factor) *
    let mut node = self.factor();

    let mut current_token = self.get_current_token();
    while current_token == Multiply || current_token == Divide {
      self.consume(&current_token);
      node = Box::new(BinOpNode::new(node, self.factor(), current_token));
      current_token = self.get_current_token();
    }
    node
  }
  fn expr(&mut self) -> Box<Node> {
    // term ((Plus | Minus) term))*
    let mut node = self.term();

    let mut current_token = self.get_current_token();
    while current_token == Plus || current_token == Minus {
      self.consume(&current_token);
      node = Box::new(BinOpNode::new(node, self.factor(), current_token));
      current_token = self.get_current_token();
    }
    node
  }
  pub fn parse(&mut self) -> Box<Node> {
    self.expr()
  }
}
