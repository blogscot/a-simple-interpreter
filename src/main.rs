use std::fmt;

#[derive(Clone, Debug, PartialEq)]
enum TokenType {
    Integer(i32),
    Plus,
    Minus,
    EOF,
}

use TokenType::*;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Integer(value) => format!("Integer, {}", value),
            Plus => "Plus".into(),
            Minus => "Minus".into(),
            EOF => "EOF".into(),
        };
        write!(f, "{}", output)
    }
}

#[derive(Clone, Debug)]
struct Token {
    token_type: TokenType,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({})", self.token_type)
    }
}

#[derive(Clone)]
struct Interpreter {
    text: String,
    position: usize,
    current_token: Option<Token>,
}

impl Interpreter {
    pub fn new(text: String) -> Self {
        Interpreter {
            text,
            position: 0,
            current_token: None,
        }
    }
    pub fn get_next_token(&mut self) -> Option<Token> {
        if self.position > self.text.len() - 1 {
            return Some(Token { token_type: EOF });
        }

        let current_char = self.text.as_bytes()[self.position] as char;
        match current_char {
            char if char.is_digit(10) => {
                self.position += 1;
                Some(Token {
                    token_type: Integer(current_char.to_digit(10).unwrap() as i32),
                })
            }
            ' ' => {
                self.position += 1;
                return self.get_next_token();
            }
            '+' => {
                self.position += 1;
                Some(Token { token_type: Plus })
            }
            '-' => {
                self.position += 1;
                Some(Token { token_type: Minus })
            }
            _ => panic!(format!("Invalid token found: {}", current_char)),
        }
    }
    fn eat(&mut self, token: Token) {
        let current_token = self.clone().current_token.unwrap();
        if current_token.token_type == token.token_type {
            self.current_token = self.get_next_token();
        } else {
            panic!("Token error: eat!")
        }
    }
    fn expr(&mut self) -> i32 {
        self.current_token = self.get_next_token();
        let mut left = 0;
        let mut right = 0;
        let mut operator = "unknown";

        let token = self.clone().current_token.unwrap();
        if let Integer(value) = token.token_type {
            left = value;
            self.eat(token);
        }

        let token = self.clone().current_token.unwrap();
        if token.token_type == Plus {
            operator = "plus";
            self.eat(token);
        } else if token.token_type == Minus {
            operator = "minus";
            self.eat(token);
        }

        let token = self.clone().current_token.unwrap();
        if let Integer(value) = token.token_type {
            right = value;
            self.eat(token);
        }

        match operator.as_ref() {
            "plus" => left + right,
            "minus" => left - right,
            _ => panic!("Unknown operator encountered!"),
        }
    }
}

fn main() {
    let mut interpreter = Interpreter::new("4 - 7".into());
    let result = interpreter.expr();

    println!("{}", result);
}
