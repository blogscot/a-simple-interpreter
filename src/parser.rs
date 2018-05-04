use token::Token;
use token::Token::*;

use lexer::Lexer;
use node::*;

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
  fn program(&mut self) -> Box<Node> {
    // "program : compound_statement DOT"
    let node = self.compound_statement();
    self.consume(&Period);
    node
  }
  fn compound_statement(&mut self) -> Box<Node> {
    // "compound_statement : BEGIN statement_list END"
    self.consume(&Begin);
    let nodes = self.statement_list();
    self.consume(&End);

    let root = Box::new(CompoundNode::new(nodes));
    root
  }
  fn statement_list(&mut self) -> Vec<Box<Node>> {
    // "statement_list : statement
    //                   | statement SEMI statement_list"
    let node = self.statement();
    let mut results = vec![node];

    while self.get_current_token() == Semi {
      self.consume(&Semi);
      results.append(&mut vec![self.statement()]);

      if let Id(_) = self.get_current_token() {
        panic!(
          "Invalid token in statement list: {}",
          self.get_current_token()
        )
      }
    }
    return results;
  }
  fn statement(&mut self) -> Box<Node> {
    // "statement : compound_statement
    //            | assign_statement
    //            | empty"
    match self.get_current_token() {
      Begin => self.compound_statement(),
      Id(_) => self.assignment_statement(),
      _ => self.empty(),
    }
  }
  fn assignment_statement(&mut self) -> Box<Node> {
    // "assignment_statement : variable ASSIGN expr"
    let left = self.variable();
    let current_token = self.get_current_token();
    self.consume(&Assign);
    let right = self.expr();
    let node = AssignNode::new(left, right, current_token);
    Box::new(node)
  }
  fn variable(&mut self) -> Box<Node> {
    // "variable : ID"
    let current_token = self.get_current_token();
    if let Id(_) = current_token {
      self.consume(&current_token);
      let node = VarNode::new(current_token);
      Box::new(node)
    } else {
      panic!("Invalid variable: {}", current_token);
    }
  }
  fn empty(&self) -> Box<Node> {
    Box::new(NoOpNode {})
  }
  fn factor(&mut self) -> Box<Node> {
    // factor : Plus factor
    //        | Minus factor
    //        | Integer
    //        | LParen expr RParen
    //        | variable
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
      self.consume(&RParen);
      node
    } else {
      self.variable()
    }
  }
  fn term(&mut self) -> Box<Node> {
    // factor ((Multiply | Divide) factor)*
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
    let node = self.program();
    let current_token = self.get_current_token();
    if current_token != EOF {
      panic!("Unexpected token found at end of file: {}", current_token);
    }
    node
  }
}
