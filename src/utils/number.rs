use regex::Regex;
use std::ops;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseNumberError {
  kind: NumberErrorKind,
}

#[derive(Debug)]
#[allow(dead_code)]
enum NumberErrorKind {
  Empty,
  InvalidNumber,
}

#[derive(Debug, PartialEq)]
pub enum Number {
  Nil,
  Int(i32),
  Real(f32),
}

pub type NumberResult = Result<Number, String>;

use self::Number::*;

fn convert(text: &str) -> Number {
  let re = Regex::new(r"^Int\((?P<int>[-+]?\d+)\)|^Real\((?P<real>[-+]?\d+\.\d*)\)").unwrap();

  let cap = re.captures(text).unwrap();
  let int_as_str = cap.name("int").map_or("", |m| m.as_str());
  let real_as_str = cap.name("real").map_or("", |m| m.as_str());

  if !int_as_str.is_empty() {
    let value = int_as_str.parse::<i32>().unwrap();
    Number::Int(value)
  } else if !real_as_str.is_empty() {
    let value = real_as_str.parse::<f32>().unwrap();
    Number::Real(value)
  } else {
    Number::Nil
  }
}

impl FromStr for Number {
  type Err = ParseNumberError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(convert(s))
  }
}

impl ToString for Number {
  fn to_string(&self) -> String {
    match self {
      Number::Nil => "Nil".into(),
      Number::Int(value) => format!("Int({})", value),
      Number::Real(value) => format!("Real({})", value),
    }
  }
}

impl ops::Neg for Number {
  type Output = Number;

  fn neg(self) -> Self {
    match self {
      Nil => Nil,
      Int(value) => Int(-value),
      Real(value) => Real(-value),
    }
  }
}

impl ops::Add for Number {
  type Output = Number;
  fn add(self, rhs: Number) -> Number {
    match self {
      Nil => rhs,
      Int(left) => match rhs {
        Nil => self,
        Int(right) => Int(left + right),
        Real(right) => panic!("Invalid addition, {} and {}", left, right),
      },
      Real(left) => match rhs {
        Nil => self,
        Int(right) => panic!("Invalid addition, {} and {}", left, right),
        Real(right) => Real(left + right),
      },
    }
  }
}

impl ops::Mul for Number {
  type Output = Number;
  fn mul(self, rhs: Number) -> Number {
    match self {
      Nil => rhs,
      Int(left) => match rhs {
        Nil => self,
        Int(right) => Int(left * right),
        Real(right) => panic!("Invalid multiplication, {} and {}", left, right),
      },
      Real(left) => match rhs {
        Nil => self,
        Int(right) => panic!("Invalid multiplication, {} and {}", left, right),
        Real(right) => Real(left * right),
      },
    }
  }
}

impl ops::Sub for Number {
  type Output = Number;
  fn sub(self, rhs: Number) -> Number {
    match self {
      Nil => rhs,
      Int(left) => match rhs {
        Nil => self,
        Int(right) => Int(left - right),
        Real(right) => panic!("Invalid subtraction, {} and {}", left, right),
      },
      Real(left) => match rhs {
        Nil => self,
        Int(right) => panic!("Invalid subtraction, {} and {}", left, right),
        Real(right) => Real(left - right),
      },
    }
  }
}

impl ops::Div for Number {
  type Output = Number;
  fn div(self, rhs: Number) -> Number {
    match self {
      Nil => rhs,
      Int(left) => match rhs {
        Nil => self,
        Int(right) => Int(left / right),
        Real(right) => panic!("Invalid division, {} and {}", left, right),
      },
      Real(left) => match rhs {
        Nil => self,
        Int(right) => panic!("Invalid division, {} and {}", left, right),
        Real(right) => Real(left / right),
      },
    }
  }
}

impl From<i32> for Number {
  fn from(num: i32) -> Self {
    Number::Int(num)
  }
}

impl From<f32> for Number {
  fn from(num: f32) -> Self {
    Number::Real(num)
  }
}
