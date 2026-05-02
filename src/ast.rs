// #![allow(dead_code)]

use std::fmt;

use crate::{
  error::{SyntaxError, UserDefinedError},
  lexer::{Delimiter, Identifier, Keyword, Literal, Operator, Token},
};

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
// #[allow(clippy::recursive_drop)]
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

impl Drop for Statement {
  fn drop(&mut self) {
    let mut stack: Vec<Box<Statement>> = Vec::new();
    match self {
      Statement::Sequence(left, right) => {
        stack.push(std::mem::replace(left, Box::new(Statement::Skip)));
        stack.push(std::mem::replace(right, Box::new(Statement::Skip)));
      }
      Statement::If(_, then_block, else_block) => {
        stack.push(std::mem::replace(then_block, Box::new(Statement::Skip)));
        stack.push(std::mem::replace(else_block, Box::new(Statement::Skip)));
      }
      Statement::While(_, while_block) => {
        stack.push(std::mem::replace(while_block, Box::new(Statement::Skip)));
      }
      _ => {}
    }
    while let Some(mut node) = stack.pop() {
      match &mut *node {
        Statement::Sequence(left, right) => {
          stack.push(std::mem::replace(left, Box::new(Statement::Skip)));
          stack.push(std::mem::replace(right, Box::new(Statement::Skip)));
        }
        Statement::If(_, then_b, else_b) => {
          stack.push(std::mem::replace(then_b, Box::new(Statement::Skip)));
          stack.push(std::mem::replace(else_b, Box::new(Statement::Skip)));
        }
        Statement::While(_, body) => {
          stack.push(std::mem::replace(body, Box::new(Statement::Skip)));
        }
        _ => {}
      }
    }
  }
}

fn parse_factor(input: &[Token]) -> Result<(AExpression, usize), UserDefinedError> {
  if input.is_empty() {
    return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd));
  }
  match &input[0] {
    Token::Lit(Literal::Int(i)) => Ok((AExpression::Int(*i), 1)),
    Token::Idf(Identifier::Variable(s)) => Ok((AExpression::Var(s.clone()), 1)),
    Token::Op(Operator::Sub) => {
      if input.len() < 2 {
        Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd))
      } else {
        match &input[1] {
          Token::Lit(Literal::Int(i)) => Ok((AExpression::Int(-i), 2)),
          _ => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
        }
      }
    }
    Token::Del(Delimiter::LParen) => {
      if input.len() < 2 {
        Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd))
      } else {
        let (left, consumed) = parse_aexpr(&input[1..])?;
        match input.get(1 + consumed) {
          Some(Token::Del(Delimiter::RParen)) => Ok((left, consumed + 2)),
          Some(_) => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
          None => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
        }
      }
    }
    _ => Err(UserDefinedError::Syntax(SyntaxError::NoMatch)),
  }
}

fn parse_term(input: &[Token]) -> Result<(AExpression, usize), UserDefinedError> {
  let (mut left, mut consumed) = parse_factor(input)?;

  while let Some(Token::Op(Operator::Mul)) = input.get(consumed) {
    let (right, right_skip) = parse_factor(&input[consumed + 1..])?;
    left = AExpression::Mul(Box::new(left), Box::new(right));
    consumed += 1 + right_skip;
  }
  // loop {
  //   match input.get(consumed) {
  //     Some(Token::Op(Operator::Mul)) => {
  //       let (right, right_skip) = parse_factor(&input[consumed + 1..])?;
  //       left = AExpression::Mul(Box::new(left), Box::new(right));
  //       consumed += 1 + right_skip;
  //     }
  //     _ => break,
  //   }
  // }

  Ok((left, consumed))
}

fn parse_aexpr(input: &[Token]) -> Result<(AExpression, usize), UserDefinedError> {
  let (mut left, mut consumed) = parse_term(input)?;

  loop {
    match input.get(consumed) {
      Some(Token::Op(Operator::Add)) => {
        let (right, right_skip) = parse_term(&input[consumed + 1..])?;
        left = AExpression::Add(Box::new(left), Box::new(right));
        consumed += 1 + right_skip;
      }
      Some(Token::Op(Operator::Sub)) => {
        let (right, right_skip) = parse_term(&input[consumed + 1..])?;
        left = AExpression::Sub(Box::new(left), Box::new(right));
        consumed += 1 + right_skip;
      }
      _ => break,
    }
  }

  Ok((left, consumed))
}

fn parse_bfactor(input: &[Token]) -> Result<(BExpression, usize), UserDefinedError> {
  if input.is_empty() {
    return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd));
  }
  match &input[0] {
    Token::Lit(Literal::True) => Ok((BExpression::True, 1)),
    Token::Lit(Literal::False) => Ok((BExpression::False, 1)),
    Token::Del(Delimiter::LParen) => {
      if input.len() < 2 {
        Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd))
      } else {
        let (left, consumed) = parse_bexpr(&input[1..])?;
        match input.get(1 + consumed) {
          Some(Token::Del(Delimiter::RParen)) => Ok((left, consumed + 2)),
          Some(_) => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
          None => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
        }
      }
    }
    _ => Err(UserDefinedError::Syntax(SyntaxError::NoMatch)),
  }
}

fn parse_bcomp(input: &[Token]) -> Result<(BExpression, usize), UserDefinedError> {
  if input.is_empty() {
    return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd));
  }

  match &input[0] {
    Token::Op(Operator::Not) => {
      if input.len() < 2 {
        Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd))
      } else {
        let (left, consumed) = parse_bcomp(&input[1..])?;
        Ok((BExpression::Not(Box::new(left)), consumed + 1))
      }
    }
    _ => match parse_aexpr(input) {
      Ok((left, consumed_left)) => match input.get(consumed_left) {
        Some(Token::Op(Operator::Equ)) => {
          if input.len() < consumed_left + 2 {
            Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd))
          } else {
            match parse_aexpr(&input[consumed_left + 1..]) {
              Ok((right, consumed_right)) => Ok((
                BExpression::Equ(left, right),
                (consumed_left + 1 + consumed_right),
              )),
              _ => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
            }
          }
        }
        Some(Token::Op(Operator::Leq)) => {
          if input.len() < consumed_left + 2 {
            Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd))
          } else {
            match parse_aexpr(&input[consumed_left + 1..]) {
              Ok((right, consumed_right)) => Ok((
                BExpression::Leq(left, right),
                (consumed_left + 1 + consumed_right),
              )),
              _ => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
            }
          }
        }
        Some(_) => parse_bfactor(input),
        None => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
      },
      Err(UserDefinedError::Syntax(SyntaxError::NoMatch)) => parse_bfactor(input),
      Err(e) => Err(e),
    },
  }
}

fn parse_bexpr(input: &[Token]) -> Result<(BExpression, usize), UserDefinedError> {
  let (mut left, mut consumed) = parse_bcomp(input)?;

  while let Some(Token::Op(Operator::And)) = input.get(consumed) {
    let (right, right_skip) = parse_bcomp(&input[consumed + 1..])?;
    left = BExpression::And(Box::new(left), Box::new(right));
    consumed += 1 + right_skip;
  }

  Ok((left, consumed))
}

fn parse_stmt(input: &[Token]) -> Result<(Statement, usize), UserDefinedError> {
  if input.is_empty() {
    return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd));
  }

  let (stmt, consumed) = match input[0] {
    Token::Idf(Identifier::Variable(_)) => match input.get(1) {
      Some(Token::Op(Operator::Assign)) => parse_assign(input),
      _ => Err(UserDefinedError::Syntax(SyntaxError::NoMatch)),
    },
    Token::Kw(Keyword::If) => parse_if(input),
    Token::Kw(Keyword::While) => parse_while(input),
    Token::Kw(Keyword::Skip) => Ok((Statement::Skip, 1)),
    Token::Kw(Keyword::Then) => Err(UserDefinedError::Syntax(SyntaxError::ThenWithoutIf)),
    Token::Kw(Keyword::Else) => Err(UserDefinedError::Syntax(SyntaxError::ElseWithoutIf)),
    Token::Kw(Keyword::Do) => Err(UserDefinedError::Syntax(SyntaxError::DoWithoutWhile)),
    Token::Kw(Keyword::End) => Err(UserDefinedError::Syntax(SyntaxError::EndWithoutBlock)),
    _ => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
  }?;

  match input.get(consumed) {
    Some(Token::Op(Operator::Semicolon)) => {
      let (right, right_consumed) = parse_stmt(&input[consumed + 1..])?;
      Ok((
        Statement::Sequence(Box::new(stmt), Box::new(right)),
        consumed + 1 + right_consumed,
      ))
    }
    _ => Ok((stmt, consumed)),
  }
}

fn parse_assign(input: &[Token]) -> Result<(Statement, usize), UserDefinedError> {
  match &input.first() {
    Some(Token::Idf(Identifier::Variable(varname))) => match &input.get(1) {
      Some(Token::Op(Operator::Assign)) => {
        if input.len() < 3 {
          Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd))
        } else {
          let (expression, consumed) = parse_aexpr(&input[2..])?;
          Ok((Statement::Assign(varname.clone(), expression), consumed + 2))
        }
      }
      _ => Err(UserDefinedError::Syntax(SyntaxError::InvalidAssign)),
    },
    _ => Err(UserDefinedError::Syntax(SyntaxError::InvalidAssign)),
  }
}

fn parse_if(input: &[Token]) -> Result<(Statement, usize), UserDefinedError> {
  let (cond, cond_consumed) = parse_bexpr(&input[1..])?;
  let mut cursor = 1 + cond_consumed;

  match input.get(cursor) {
    Some(Token::Kw(Keyword::Then)) => cursor += 1,
    Some(_) => return Err(UserDefinedError::Syntax(SyntaxError::ExpectedThen)),
    None => return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
  }

  let (then_branch, then_consumed) = parse_stmt(&input[cursor..])?;
  cursor += then_consumed;

  match input.get(cursor) {
    Some(Token::Kw(Keyword::Else)) => cursor += 1,
    Some(_) => return Err(UserDefinedError::Syntax(SyntaxError::ExpectedElse)),
    None => return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
  }

  let (else_branch, else_consumed) = parse_stmt(&input[cursor..])?;
  cursor += else_consumed;

  match input.get(cursor) {
    Some(Token::Kw(Keyword::End)) => cursor += 1,
    Some(_) => return Err(UserDefinedError::Syntax(SyntaxError::ExpectedEnd)),
    None => return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
  }

  Ok((
    Statement::If(cond, Box::new(then_branch), Box::new(else_branch)),
    cursor,
  ))
}

fn parse_while(input: &[Token]) -> Result<(Statement, usize), UserDefinedError> {
  let (cond, cond_consumed) = parse_bexpr(&input[1..])?;
  let mut cursor = 1 + cond_consumed;

  match input.get(cursor) {
    Some(Token::Kw(Keyword::Do)) => cursor += 1,
    Some(_) => return Err(UserDefinedError::Syntax(SyntaxError::ExpectedDo)),
    None => return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
  }

  let (body, body_consumed) = parse_stmt(&input[cursor..])?;
  cursor += body_consumed;

  match input.get(cursor) {
    Some(Token::Kw(Keyword::End)) => cursor += 1,
    Some(_) => return Err(UserDefinedError::Syntax(SyntaxError::ExpectedEnd)),
    None => return Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
  }

  Ok((Statement::While(cond, Box::new(body)), cursor))
}

pub fn parse(input: &[Token]) -> Result<Statement, UserDefinedError> {
  let (stmt, consumed) = parse_stmt(input)?;

  match input.get(consumed) {
    Some(Token::End) => Ok(stmt),
    Some(_) => Err(UserDefinedError::Syntax(SyntaxError::UnconsumedTokens)),
    None => Err(UserDefinedError::Syntax(SyntaxError::UndefinedEnd)),
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
