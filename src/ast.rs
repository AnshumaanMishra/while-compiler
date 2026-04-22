#![allow(dead_code)]

use std::fmt;

// Arithmetic Expressions
#[derive(Debug, PartialEq, Clone)]
pub enum AExpression {
  Int(i64),
  Var(String),
  Add(Box<AExpression>, Box<AExpression>),
  Sub(Box<AExpression>, Box<AExpression>),
  Mul(Box<AExpression>, Box<AExpression>),
}

impl fmt::Display for AExpression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AExpression::Int(n) => write!(f, "{}", n),
      AExpression::Var(s) => write!(f, "{}", s),
      AExpression::Add(a, b) => write!(f, "{} + {}", a, b),
      AExpression::Sub(a, b) => write!(f, "{} - {}", a, b),
      AExpression::Mul(a, b) => write!(f, "{} * {}", a, b),
    }
  }
}

// Boolean Expressions
#[derive(Debug, PartialEq, Clone)]
pub enum BExpression {
  True,
  False,
  Equ(AExpression, AExpression),
  Leq(AExpression, AExpression),
  Not(Box<BExpression>),
  And(Box<BExpression>, Box<BExpression>),
}

impl fmt::Display for BExpression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BExpression::True => write!(f, "true"),
      BExpression::False => write!(f, "false"),
      BExpression::Equ(a, b) => write!(f, "{} = {}", a, b),
      BExpression::Leq(a, b) => write!(f, "{} <= {}", a, b),
      BExpression::Not(a) => write!(f, "!{}", a),
      BExpression::And(a, b) => write!(f, "{} & {}", a, b),
    }
  }
}

// TopLevel Statement
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
  Assign(String, AExpression),
  Skip,
  Sequence(Box<Statement>, Box<Statement>),
  If(BExpression, Box<Statement>, Box<Statement>),
  While(BExpression, Box<Statement>),
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Statement::Assign(s, a) => write!(f, "{} := {}", s, a),
      Statement::Skip => write!(f, "Skip",),
      Statement::Sequence(a, b) => write!(f, "{}; {}", a, b),
      Statement::If(c, s1, s2) => write!(f, "if {} then {} else {}", c, s1, s2),
      Statement::While(c, s) => write!(f, "while {} do {}", c, s),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_aexpression_display() {
    // Build an AST for: 5 + 10
    let expr = AExpression::Add(
      Box::new(AExpression::Int(5)),
      Box::new(AExpression::Int(10)),
    );

    // Test that the Display trait formats it correctly
    assert_eq!(expr.to_string(), "5 + 10");
  }

  #[test]
  fn test_bexpression_display() {
    // Build an AST for: x <= 100
    let expr = BExpression::Leq(AExpression::Var("x".to_string()), AExpression::Int(100));

    assert_eq!(expr.to_string(), "x <= 100");
  }

  #[test]
  fn test_statement_display() {
    // Build an AST for: if true then x := 1 else Skip
    let statement = Statement::If(
      BExpression::True,
      Box::new(Statement::Assign("x".to_string(), AExpression::Int(1))),
      Box::new(Statement::Skip),
    );

    assert_eq!(statement.to_string(), "if true then x := 1 else Skip");
  }

  #[test]
  fn test_complex_sequence() {
    // Build an AST for: x := 5; y := x + 1
    let seq = Statement::Sequence(
      Box::new(Statement::Assign("x".to_string(), AExpression::Int(5))),
      Box::new(Statement::Assign(
        "y".to_string(),
        AExpression::Add(
          Box::new(AExpression::Var("x".to_string())),
          Box::new(AExpression::Int(1)),
        ),
      )),
    );

    assert_eq!(seq.to_string(), "x := 5; y := x + 1");
  }

  #[test]
  fn test_ast_equality() {
    // This tests the #[derive(PartialEq)] macro.
    let ast1 = AExpression::Int(42);
    let ast2 = AExpression::Int(42);

    assert_eq!(ast1, ast2);
  }
}
