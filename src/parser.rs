use token::Token;
use token::TokenType;
use token::TokenType::*;

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
  fn get_token_type(&self) -> TokenType {
    self.get_current_token().get_type()
  }
  ///
  /// Verifies the token type matches the current token type.
  /// If valid the next token is saved.
  ///
  fn consume(&mut self, token_type: &TokenType) {
    let current_token = self.get_current_token();

    if current_token.get_type() == *token_type {
      self.current_token = self.lexer.get_next_token();
    } else {
      panic!(format!("consume: token error: {}", current_token));
    }
  }
  fn factor(&mut self) -> Box<Node> {
    // factor : <PLUS | Minus) Factor | Integer | LParen expr RParen
    let token_type = self.get_token_type();

    if token_type == Plus || token_type == Minus {
      self.consume(&token_type);
      let node = UnaryOpNode::new(token_type, self.factor());
      Box::new(node)
    } else if let Integer(value) = token_type {
      self.consume(&token_type);
      Box::new(NumNode::new(value))
    } else if let LParen = token_type {
      self.consume(&token_type);
      let node = self.expr();
      self.consume(&TokenType::RParen);
      node
    } else {
      panic!(format!("Invalid factor: {}", token_type));
    }
  }
  fn term(&mut self) -> Box<Node> {
    // factor ((Multiply | Divide) factor) *
    let mut node = self.factor();

    let mut token_type = self.get_token_type();
    while token_type == Multiply || token_type == Divide {
      self.consume(&token_type);
      node = Box::new(BinOpNode::new(node, self.factor(), token_type));
      token_type = self.get_token_type();
    }
    node
  }
  fn expr(&mut self) -> Box<Node> {
    // term ((Plus | Minus) term))*
    let mut node = self.term();

    let mut token_type = self.get_token_type();
    while token_type == Plus || token_type == Minus {
      self.consume(&token_type);
      node = Box::new(BinOpNode::new(node, self.factor(), token_type));
      token_type = self.get_token_type();
    }
    node
  }
  pub fn parse(&mut self) -> Box<Node> {
    self.expr()
  }
}
