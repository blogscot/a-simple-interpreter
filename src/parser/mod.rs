use lexer::token::Token;
use lexer::token::Token::*;

use ast::node::*;
use lexer::Lexer;

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
      panic!(
        "Unexpected token error: expected {}, received {}",
        token_type, current_token
      );
    }
  }
  fn program(&mut self) -> Box<Node> {
    // "program : Program variable Semi block Period"
    self.consume(&Program);
    let VarNode { identifier } = *self.variable();
    self.consume(&Semi);
    let block = self.block();
    let node = ProgramNode::new(identifier, block);
    self.consume(&Period);
    Box::new(node)
  }
  fn block(&mut self) -> Box<Node> {
    // "block : declarations compound_statement"
    let declarations = self.declarations();
    let compound_statement = self.compound_statement();
    let node = BlockNode::new(declarations, compound_statement);
    Box::new(node)
  }
  fn compound_statement(&mut self) -> Box<Node> {
    // "compound_statement : Begin statement_list End"
    self.consume(&Begin);
    let nodes = self.statement_list();
    self.consume(&End);

    Box::new(CompoundNode::new(nodes))
  }
  fn declarations(&mut self) -> Vec<Box<Node>> {
    // "declarations : Var (variable_declaration Semi)+ (procedure_declaration)*
    //               | (procedure_declaration)*
    //               | empty"
    let mut declarations: Vec<Box<Node>> = vec![];
    if self.get_current_token() == Var {
      self.consume(&Var);
      let mut current_token = self.get_current_token();
      while let Id(_) = current_token {
        declarations.extend(self.variable_declaration());
        current_token = self.get_current_token();
        self.consume(&current_token);
        current_token = self.get_current_token();
      }
    }
    while self.get_current_token() == Procedure {
      declarations.push(self.procedure_declaration());
    }
    declarations
  }
  fn procedure_declaration(&mut self) -> Box<Node> {
    // "procedure_declaration :
    //    (Procedure Id (LParen formal_parameter_list RParen)? Semi Block Semi)*"
    self.consume(&Procedure);
    let proc_name = self.get_current_token();
    self.consume(&proc_name);
    self.consume(&LParen);
    let params = self.formal_parameter_list();
    self.consume(&RParen);
    self.consume(&Semi);
    let block = self.block();
    self.consume(&Semi);
    Box::new(ProcedureNode::new(proc_name, params, block))
  }
  fn formal_parameter_list(&mut self) -> Vec<Box<Node>> {
    // "formal_parameter_list : formal_parameters
    //                        | formal_parameter Semi formal_parameter_list"
    let mut parameter_nodes: Vec<VarNode> = Vec::new();
    let mut identifier = self.get_current_token();
    self.consume(&identifier);

    parameter_nodes.push(VarNode::new(identifier));
    while self.get_current_token() == Comma {
      self.consume(&Comma);
      identifier = self.get_current_token();
      self.consume(&identifier);
      parameter_nodes.push(VarNode::new(identifier));
    }

    self.consume(&Colon);

    let type_node = self.type_spec();
    let mut parameter_list: Vec<Box<Node>> = vec![];
    for node in parameter_nodes {
      let parameter_node = ParameterNode::new(node, type_node.clone());
      parameter_list.push(Box::new(parameter_node));
    }
    parameter_list
  }
  fn variable_declaration(&mut self) -> Vec<Box<Node>> {
    // "variable_declaration : Id (Comma Id)* Colon type_spec"
    let mut var_nodes: Vec<VarNode> = Vec::new();
    let mut identifier = self.get_current_token();
    self.consume(&identifier);

    var_nodes.push(VarNode::new(identifier));
    while self.get_current_token() == Comma {
      self.consume(&Comma);
      identifier = self.get_current_token();
      self.consume(&identifier);
      var_nodes.push(VarNode::new(identifier));
    }

    self.consume(&Colon);

    let type_node = self.type_spec();
    let mut var_declarations: Vec<Box<Node>> = vec![];
    for node in var_nodes {
      let declaration = DeclarationNode::new(node, type_node.clone());
      var_declarations.push(Box::new(declaration));
    }
    var_declarations
  }
  fn type_spec(&mut self) -> TypeNode {
    // "type_spec : Integer
    //              Real"
    let current_token = self.get_current_token();
    match current_token {
      Integer | Real => {
        self.consume(&current_token);
        TypeNode::new(current_token)
      }
      token => panic!("Unknown token type found {}", token),
    }
  }
  fn statement_list(&mut self) -> Vec<Box<Node>> {
    // "statement_list : statement
    //                 | statement Semi statement_list"
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
    results
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
    // "assignment_statement : variable Assign expr"
    let left = self.variable();
    let current_token = self.get_current_token();
    self.consume(&Assign);
    let right = self.expr();
    let node = AssignNode::new(left, right, current_token);
    Box::new(node)
  }
  fn variable(&mut self) -> Box<VarNode> {
    // "variable : Id"
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
    let mut current_token = self.get_current_token();

    match current_token {
      Plus | Minus => {
        self.consume(&current_token);
        let node = UnaryOpNode::new(current_token, self.factor());
        Box::new(node)
      }
      IntegerConst(value) => {
        current_token = self.get_current_token();
        self.consume(&current_token);
        Box::new(IntegerNumNode::new(value.parse::<i32>().unwrap()))
      }
      RealConst(value) => {
        current_token = self.get_current_token();
        self.consume(&current_token);
        Box::new(RealNumNode::new(value.parse::<f32>().unwrap()))
      }
      LParen => {
        self.consume(&current_token);
        let node = self.expr();
        self.consume(&RParen);
        node
      }
      _ => self.variable(),
    }
  }
  fn term(&mut self) -> Box<Node> {
    // factor ((Multiply | Divide) factor)*
    let mut node = self.factor();
    let mut current_token = self.get_current_token();

    while current_token == Multiply || current_token == IntegerDivision
      || current_token == RealDivision
    {
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
      node = Box::new(BinOpNode::new(node, self.term(), current_token));
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
