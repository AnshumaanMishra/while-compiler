// #![allow(dead_code)]

use pretty::RcDoc;
use serde::Serialize;
use std::fmt;

use crate::{
  error::{SyntaxError, UserDefinedError},
  lexer::{Delimiter, Identifier, Keyword, Literal, Operator, Token},
};

// Arithmetic Expressions
#[derive(Debug, PartialEq, Clone, Serialize)]
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
#[derive(Debug, PartialEq, Clone, Serialize)]
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
#[derive(Debug, PartialEq, Clone, Serialize)]
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

impl AExpression {
  pub fn to_doc<'a>(&'a self) -> RcDoc<'a, ()> {
    match self {
      AExpression::Int(n) => RcDoc::as_string(n),
      AExpression::Var(s) => RcDoc::as_string(s),
      AExpression::Add(l, r) => RcDoc::text("(")
        .append(l.to_doc())
        .append(RcDoc::text(" + "))
        .append(r.to_doc())
        .append(RcDoc::text(")")),
      AExpression::Sub(l, r) => RcDoc::text("(")
        .append(l.to_doc())
        .append(RcDoc::text(" - "))
        .append(r.to_doc())
        .append(RcDoc::text(")")),
      AExpression::Mul(l, r) => l.to_doc().append(RcDoc::text(" * ")).append(r.to_doc()),
    }
  }
}

impl BExpression {
  pub fn to_doc<'a>(&'a self) -> RcDoc<'a, ()> {
    match self {
      BExpression::True => RcDoc::text("true"),
      BExpression::False => RcDoc::text("false"),
      BExpression::Equ(l, r) => l.to_doc().append(RcDoc::text(" = ")).append(r.to_doc()),
      BExpression::Leq(l, r) => l.to_doc().append(RcDoc::text(" <= ")).append(r.to_doc()),
      BExpression::Not(b) => RcDoc::text("not (")
        .append(b.to_doc())
        .append(RcDoc::text(")")),
      BExpression::And(l, r) => l.to_doc().append(RcDoc::text(" and ")).append(r.to_doc()),
    }
  }
}

impl Statement {
  pub fn to_doc<'a>(&'a self) -> RcDoc<'a, ()> {
    match self {
      Statement::Skip => RcDoc::text("skip"),
      Statement::Assign(var, expr) => RcDoc::as_string(var)
        .append(RcDoc::text(" := "))
        .append(expr.to_doc()),
      Statement::Sequence(s1, s2) => s1
        .to_doc()
        .append(RcDoc::text(";"))
        .append(RcDoc::hardline())
        .append(s2.to_doc()),
      Statement::If(cond, s1, s2) => RcDoc::text("if ")
        .append(cond.to_doc())
        .append(RcDoc::text(" then"))
        .append(RcDoc::hardline().append(s1.to_doc()).nest(2))
        .append(RcDoc::hardline())
        .append(RcDoc::text("else"))
        .append(RcDoc::hardline().append(s2.to_doc()).nest(2))
        .append(RcDoc::hardline())
        .append(RcDoc::text("end")),
      Statement::While(cond, body) => RcDoc::text("while ")
        .append(cond.to_doc())
        .append(RcDoc::text(" do"))
        .append(RcDoc::hardline().append(body.to_doc()).nest(2))
        .append(RcDoc::hardline())
        .append(RcDoc::text("end")),
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
      Err(_) => parse_bfactor(input),
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
  use crate::lexer::{Delimiter, Identifier, Keyword, Literal, Operator, Token};

  // -------------------------------------------------------------------------
  // Token stream builders — keep tests readable
  // -------------------------------------------------------------------------

  fn int(n: i64) -> Token {
    Token::Lit(Literal::Int(n))
  }
  fn var(s: &str) -> Token {
    Token::Idf(Identifier::Variable(s.to_string()))
  }
  fn kw(k: Keyword) -> Token {
    Token::Kw(k)
  }
  fn op(o: Operator) -> Token {
    Token::Op(o)
  }
  fn del(d: Delimiter) -> Token {
    Token::Del(d)
  }
  fn eof() -> Token {
    Token::End
  }

  // Appends Token::End so every stream is valid for parse()
  fn stream(mut tokens: Vec<Token>) -> Vec<Token> {
    tokens.push(eof());
    tokens
  }

  // -------------------------------------------------------------------------
  // Display — AExpression
  // -------------------------------------------------------------------------

  #[test]
  fn display_aexpr_int() {
    assert_eq!(AExpression::Int(42).to_string(), "42");
  }

  #[test]
  fn display_aexpr_negative_int() {
    assert_eq!(AExpression::Int(-7).to_string(), "-7");
  }

  #[test]
  fn display_aexpr_var() {
    assert_eq!(AExpression::Var("x".to_string()).to_string(), "x");
  }

  #[test]
  fn display_aexpr_add() {
    let e = AExpression::Add(Box::new(AExpression::Int(1)), Box::new(AExpression::Int(2)));
    assert_eq!(e.to_string(), "1 + 2");
  }

  #[test]
  fn display_aexpr_sub() {
    let e = AExpression::Sub(
      Box::new(AExpression::Var("x".to_string())),
      Box::new(AExpression::Int(1)),
    );
    assert_eq!(e.to_string(), "x - 1");
  }

  #[test]
  fn display_aexpr_mul() {
    let e = AExpression::Mul(Box::new(AExpression::Int(3)), Box::new(AExpression::Int(4)));
    assert_eq!(e.to_string(), "3 * 4");
  }

  #[test]
  fn display_aexpr_nested() {
    // (x + 1) * 2  stored as Mul(Add(x,1), 2)
    let e = AExpression::Mul(
      Box::new(AExpression::Add(
        Box::new(AExpression::Var("x".to_string())),
        Box::new(AExpression::Int(1)),
      )),
      Box::new(AExpression::Int(2)),
    );
    assert_eq!(e.to_string(), "x + 1 * 2");
  }

  // -------------------------------------------------------------------------
  // Display — BExpression
  // -------------------------------------------------------------------------

  #[test]
  fn display_bexpr_true() {
    assert_eq!(BExpression::True.to_string(), "true");
  }

  #[test]
  fn display_bexpr_false() {
    assert_eq!(BExpression::False.to_string(), "false");
  }

  #[test]
  fn display_bexpr_equ() {
    let e = BExpression::Equ(AExpression::Var("x".to_string()), AExpression::Int(0));
    assert_eq!(e.to_string(), "x = 0");
  }

  #[test]
  fn display_bexpr_leq() {
    let e = BExpression::Leq(AExpression::Var("x".to_string()), AExpression::Int(100));
    assert_eq!(e.to_string(), "x <= 100");
  }

  #[test]
  fn display_bexpr_not() {
    let e = BExpression::Not(Box::new(BExpression::True));
    assert_eq!(e.to_string(), "!true");
  }

  #[test]
  fn display_bexpr_and() {
    let e = BExpression::And(Box::new(BExpression::True), Box::new(BExpression::False));
    assert_eq!(e.to_string(), "true & false");
  }

  // -------------------------------------------------------------------------
  // Display — Statement
  // -------------------------------------------------------------------------

  #[test]
  fn display_stmt_skip() {
    assert_eq!(Statement::Skip.to_string(), "Skip");
  }

  #[test]
  fn display_stmt_assign() {
    let s = Statement::Assign("x".to_string(), AExpression::Int(5));
    assert_eq!(s.to_string(), "x := 5");
  }

  #[test]
  fn display_stmt_sequence() {
    let s = Statement::Sequence(
      Box::new(Statement::Assign("x".to_string(), AExpression::Int(1))),
      Box::new(Statement::Assign("y".to_string(), AExpression::Int(2))),
    );
    assert_eq!(s.to_string(), "x := 1; y := 2");
  }

  #[test]
  fn display_stmt_if() {
    let s = Statement::If(
      BExpression::True,
      Box::new(Statement::Skip),
      Box::new(Statement::Skip),
    );
    assert_eq!(s.to_string(), "if true then Skip else Skip");
  }

  #[test]
  fn display_stmt_while() {
    let s = Statement::While(BExpression::True, Box::new(Statement::Skip));
    assert_eq!(s.to_string(), "while true do Skip");
  }

  // -------------------------------------------------------------------------
  // PartialEq / Clone
  // -------------------------------------------------------------------------

  #[test]
  fn aexpr_equality() {
    assert_eq!(AExpression::Int(42), AExpression::Int(42));
    assert_ne!(AExpression::Int(42), AExpression::Int(0));
  }

  #[test]
  fn aexpr_clone() {
    let a = AExpression::Add(Box::new(AExpression::Int(1)), Box::new(AExpression::Int(2)));
    assert_eq!(a.clone(), a);
  }

  #[test]
  fn stmt_equality() {
    assert_eq!(Statement::Skip, Statement::Skip);
    assert_ne!(
      Statement::Skip,
      Statement::Assign("x".to_string(), AExpression::Int(0))
    );
  }

  // -------------------------------------------------------------------------
  // parse_factor (called indirectly via parse_aexpr → parse_term → parse_factor)
  // -------------------------------------------------------------------------

  #[test]
  fn parse_factor_int_literal() {
    // let tokens = stream(vec![int(7)]);
    // let result = parse(&tokens).unwrap();
    // Wrapping a lone int in an assignment to test via parse()
    // Test directly via parse_aexpr instead:
    let tokens = vec![int(7)];
    let (expr, consumed) = parse_aexpr(&tokens).unwrap();
    assert_eq!(expr, AExpression::Int(7));
    assert_eq!(consumed, 1);
  }

  #[test]
  fn parse_factor_variable() {
    let tokens = vec![var("x")];
    let (expr, consumed) = parse_aexpr(&tokens).unwrap();
    assert_eq!(expr, AExpression::Var("x".to_string()));
    assert_eq!(consumed, 1);
  }

  #[test]
  fn parse_factor_negative_int() {
    // -5
    let tokens = vec![op(Operator::Sub), int(5)];
    let (expr, consumed) = parse_aexpr(&tokens).unwrap();
    assert_eq!(expr, AExpression::Int(-5));
    assert_eq!(consumed, 2);
  }

  #[test]
  fn parse_factor_grouped_expr() {
    // (3 + 4)
    let tokens = vec![
      del(Delimiter::LParen),
      int(3),
      op(Operator::Add),
      int(4),
      del(Delimiter::RParen),
    ];
    let (expr, consumed) = parse_aexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      AExpression::Add(Box::new(AExpression::Int(3)), Box::new(AExpression::Int(4)))
    );
    assert_eq!(consumed, 5);
  }

  #[test]
  fn parse_factor_empty_returns_err() {
    let result = parse_aexpr(&[]);
    assert!(result.is_err());
  }

  #[test]
  fn parse_factor_lone_lparen_returns_err() {
    let tokens = vec![del(Delimiter::LParen)];
    assert!(parse_aexpr(&tokens).is_err());
  }

  #[test]
  fn parse_factor_unclosed_paren_returns_err() {
    // (3 + 4   — missing RParen
    let tokens = vec![del(Delimiter::LParen), int(3), op(Operator::Add), int(4)];
    assert!(parse_aexpr(&tokens).is_err());
  }

  // -------------------------------------------------------------------------
  // parse_term
  // -------------------------------------------------------------------------

  #[test]
  fn parse_term_single_factor() {
    let tokens = vec![int(5)];
    let (expr, consumed) = parse_aexpr(&tokens).unwrap();
    assert_eq!(expr, AExpression::Int(5));
    assert_eq!(consumed, 1);
  }

  #[test]
  fn parse_term_multiplication() {
    // 3 * 4
    let tokens = vec![int(3), op(Operator::Mul), int(4)];
    let (expr, consumed) = parse_aexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      AExpression::Mul(Box::new(AExpression::Int(3)), Box::new(AExpression::Int(4)))
    );
    assert_eq!(consumed, 3);
  }

  #[test]
  fn parse_term_mul_chain_left_associative() {
    // 2 * 3 * 4  →  Mul(Mul(2,3), 4)
    let tokens = vec![int(2), op(Operator::Mul), int(3), op(Operator::Mul), int(4)];
    let (expr, _) = parse_aexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      AExpression::Mul(
        Box::new(AExpression::Mul(
          Box::new(AExpression::Int(2)),
          Box::new(AExpression::Int(3)),
        )),
        Box::new(AExpression::Int(4)),
      )
    );
  }

  // -------------------------------------------------------------------------
  // parse_aexpr
  // -------------------------------------------------------------------------

  #[test]
  fn parse_aexpr_addition() {
    // 1 + 2
    let tokens = vec![int(1), op(Operator::Add), int(2)];
    let (expr, consumed) = parse_aexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      AExpression::Add(Box::new(AExpression::Int(1)), Box::new(AExpression::Int(2)))
    );
    assert_eq!(consumed, 3);
  }

  #[test]
  fn parse_aexpr_subtraction() {
    // x - 1
    let tokens = vec![var("x"), op(Operator::Sub), int(1)];
    let (expr, consumed) = parse_aexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      AExpression::Sub(
        Box::new(AExpression::Var("x".to_string())),
        Box::new(AExpression::Int(1)),
      )
    );
    assert_eq!(consumed, 3);
  }

  #[test]
  fn parse_aexpr_add_chain_left_associative() {
    // 1 + 2 + 3  →  Add(Add(1,2), 3)
    let tokens = vec![int(1), op(Operator::Add), int(2), op(Operator::Add), int(3)];
    let (expr, _) = parse_aexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      AExpression::Add(
        Box::new(AExpression::Add(
          Box::new(AExpression::Int(1)),
          Box::new(AExpression::Int(2)),
        )),
        Box::new(AExpression::Int(3)),
      )
    );
  }

  #[test]
  fn parse_aexpr_mul_binds_tighter_than_add() {
    // 1 + 2 * 3  →  Add(1, Mul(2,3))
    let tokens = vec![int(1), op(Operator::Add), int(2), op(Operator::Mul), int(3)];
    let (expr, _) = parse_aexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      AExpression::Add(
        Box::new(AExpression::Int(1)),
        Box::new(AExpression::Mul(
          Box::new(AExpression::Int(2)),
          Box::new(AExpression::Int(3)),
        )),
      )
    );
  }

  #[test]
  fn parse_aexpr_grouped_overrides_precedence() {
    // (1 + 2) * 3  →  Mul(Add(1,2), 3)
    let tokens = vec![
      del(Delimiter::LParen),
      int(1),
      op(Operator::Add),
      int(2),
      del(Delimiter::RParen),
      op(Operator::Mul),
      int(3),
    ];
    let (expr, _) = parse_aexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      AExpression::Mul(
        Box::new(AExpression::Add(
          Box::new(AExpression::Int(1)),
          Box::new(AExpression::Int(2)),
        )),
        Box::new(AExpression::Int(3)),
      )
    );
  }

  // -------------------------------------------------------------------------
  // parse_bfactor / parse_bcomp / parse_bexpr
  // -------------------------------------------------------------------------

  #[test]
  fn parse_bexpr_true() {
    // let tokens = vec![kw(Keyword::Skip), eof()]; // need parse context
    // test directly
    let tokens = vec![Token::Lit(Literal::True)];
    let (expr, consumed) = parse_bexpr(&tokens).unwrap();
    assert_eq!(expr, BExpression::True);
    assert_eq!(consumed, 1);
  }

  #[test]
  fn parse_bexpr_false() {
    let tokens = vec![Token::Lit(Literal::False)];
    let (expr, consumed) = parse_bexpr(&tokens).unwrap();
    assert_eq!(expr, BExpression::False);
    assert_eq!(consumed, 1);
  }

  #[test]
  fn parse_bexpr_equality() {
    // x = 0
    let tokens = vec![var("x"), op(Operator::Equ), int(0)];
    let (expr, consumed) = parse_bexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      BExpression::Equ(AExpression::Var("x".to_string()), AExpression::Int(0),)
    );
    assert_eq!(consumed, 3);
  }

  #[test]
  fn parse_bexpr_leq() {
    // x <= 10
    let tokens = vec![var("x"), op(Operator::Leq), int(10)];
    let (expr, consumed) = parse_bexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      BExpression::Leq(AExpression::Var("x".to_string()), AExpression::Int(10),)
    );
    assert_eq!(consumed, 3);
  }

  #[test]
  fn parse_bexpr_not() {
    // ! true
    let tokens = vec![op(Operator::Not), Token::Lit(Literal::True)];
    let (expr, consumed) = parse_bexpr(&tokens).unwrap();
    assert_eq!(expr, BExpression::Not(Box::new(BExpression::True)));
    assert_eq!(consumed, 2);
  }

  #[test]
  fn parse_bexpr_double_not() {
    // ! ! false
    let tokens = vec![
      op(Operator::Not),
      op(Operator::Not),
      Token::Lit(Literal::False),
    ];
    let (expr, _) = parse_bexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      BExpression::Not(Box::new(BExpression::Not(Box::new(BExpression::False))))
    );
  }

  #[test]
  fn parse_bexpr_and() {
    // true & false
    let tokens = vec![
      Token::Lit(Literal::True),
      op(Operator::And),
      Token::Lit(Literal::False),
    ];
    let (expr, consumed) = parse_bexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      BExpression::And(Box::new(BExpression::True), Box::new(BExpression::False))
    );
    assert_eq!(consumed, 3);
  }

  #[test]
  fn parse_bexpr_and_chain_left_associative() {
    // true & false & true  →  And(And(true,false), true)
    let tokens = vec![
      Token::Lit(Literal::True),
      op(Operator::And),
      Token::Lit(Literal::False),
      op(Operator::And),
      Token::Lit(Literal::True),
    ];
    let (expr, _) = parse_bexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      BExpression::And(
        Box::new(BExpression::And(
          Box::new(BExpression::True),
          Box::new(BExpression::False),
        )),
        Box::new(BExpression::True),
      )
    );
  }

  #[test]
  fn parse_bexpr_not_of_relational() {
    // !(x <= 0)
    let tokens = vec![
      op(Operator::Not),
      del(Delimiter::LParen),
      var("x"),
      op(Operator::Leq),
      int(0),
      del(Delimiter::RParen),
    ];
    let (expr, _) = parse_bexpr(&tokens).unwrap();
    assert_eq!(
      expr,
      BExpression::Not(Box::new(BExpression::Leq(
        AExpression::Var("x".to_string()),
        AExpression::Int(0),
      )))
    );
  }

  // -------------------------------------------------------------------------
  // parse (top-level) — happy paths
  // -------------------------------------------------------------------------

  #[test]
  fn parse_skip() {
    let tokens = stream(vec![kw(Keyword::Skip)]);
    let result = parse(&tokens).unwrap();
    assert_eq!(result, Statement::Skip);
  }

  #[test]
  fn parse_assign_int() {
    // x := 42
    let tokens = stream(vec![var("x"), op(Operator::Assign), int(42)]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::Assign("x".to_string(), AExpression::Int(42))
    );
  }

  #[test]
  fn parse_assign_expr() {
    // x := y + 1
    let tokens = stream(vec![
      var("x"),
      op(Operator::Assign),
      var("y"),
      op(Operator::Add),
      int(1),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::Assign(
        "x".to_string(),
        AExpression::Add(
          Box::new(AExpression::Var("y".to_string())),
          Box::new(AExpression::Int(1)),
        ),
      )
    );
  }

  #[test]
  fn parse_assign_negative_rhs() {
    // x := -5
    let tokens = stream(vec![
      var("x"),
      op(Operator::Assign),
      op(Operator::Sub),
      int(5),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::Assign("x".to_string(), AExpression::Int(-5))
    );
  }

  #[test]
  fn parse_sequence_two_stmts() {
    // x := 1 ; y := 2
    let tokens = stream(vec![
      var("x"),
      op(Operator::Assign),
      int(1),
      op(Operator::Semicolon),
      var("y"),
      op(Operator::Assign),
      int(2),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::Sequence(
        Box::new(Statement::Assign("x".to_string(), AExpression::Int(1))),
        Box::new(Statement::Assign("y".to_string(), AExpression::Int(2))),
      )
    );
  }

  #[test]
  fn parse_sequence_three_stmts_right_associative() {
    // x := 1 ; y := 2 ; z := 3
    // right-assoc: Sequence(x:=1, Sequence(y:=2, z:=3))
    let tokens = stream(vec![
      var("x"),
      op(Operator::Assign),
      int(1),
      op(Operator::Semicolon),
      var("y"),
      op(Operator::Assign),
      int(2),
      op(Operator::Semicolon),
      var("z"),
      op(Operator::Assign),
      int(3),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::Sequence(
        Box::new(Statement::Assign("x".to_string(), AExpression::Int(1))),
        Box::new(Statement::Sequence(
          Box::new(Statement::Assign("y".to_string(), AExpression::Int(2))),
          Box::new(Statement::Assign("z".to_string(), AExpression::Int(3))),
        )),
      )
    );
  }

  #[test]
  fn parse_if_statement() {
    // if true then skip else skip end
    let tokens = stream(vec![
      kw(Keyword::If),
      Token::Lit(Literal::True),
      kw(Keyword::Then),
      kw(Keyword::Skip),
      kw(Keyword::Else),
      kw(Keyword::Skip),
      kw(Keyword::End),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::If(
        BExpression::True,
        Box::new(Statement::Skip),
        Box::new(Statement::Skip),
      )
    );
  }

  #[test]
  fn parse_if_with_condition() {
    // if x <= 5 then y := 1 else y := 0 end
    let tokens = stream(vec![
      kw(Keyword::If),
      var("x"),
      op(Operator::Leq),
      int(5),
      kw(Keyword::Then),
      var("y"),
      op(Operator::Assign),
      int(1),
      kw(Keyword::Else),
      var("y"),
      op(Operator::Assign),
      int(0),
      kw(Keyword::End),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::If(
        BExpression::Leq(AExpression::Var("x".to_string()), AExpression::Int(5),),
        Box::new(Statement::Assign("y".to_string(), AExpression::Int(1))),
        Box::new(Statement::Assign("y".to_string(), AExpression::Int(0))),
      )
    );
  }

  #[test]
  fn parse_while_statement() {
    // while true do skip end
    let tokens = stream(vec![
      kw(Keyword::While),
      Token::Lit(Literal::True),
      kw(Keyword::Do),
      kw(Keyword::Skip),
      kw(Keyword::End),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::While(BExpression::True, Box::new(Statement::Skip))
    );
  }

  #[test]
  fn parse_while_with_not_condition() {
    // while !(x <= 0) do x := x - 1 end
    let tokens = stream(vec![
      kw(Keyword::While),
      op(Operator::Not),
      del(Delimiter::LParen),
      var("x"),
      op(Operator::Leq),
      int(0),
      del(Delimiter::RParen),
      kw(Keyword::Do),
      var("x"),
      op(Operator::Assign),
      var("x"),
      op(Operator::Sub),
      int(1),
      kw(Keyword::End),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::While(
        BExpression::Not(Box::new(BExpression::Leq(
          AExpression::Var("x".to_string()),
          AExpression::Int(0),
        ))),
        Box::new(Statement::Assign(
          "x".to_string(),
          AExpression::Sub(
            Box::new(AExpression::Var("x".to_string())),
            Box::new(AExpression::Int(1)),
          ),
        )),
      )
    );
  }

  #[test]
  fn parse_nested_if_in_while() {
    // while true do if true then skip else skip end end
    let tokens = stream(vec![
      kw(Keyword::While),
      Token::Lit(Literal::True),
      kw(Keyword::Do),
      kw(Keyword::If),
      Token::Lit(Literal::True),
      kw(Keyword::Then),
      kw(Keyword::Skip),
      kw(Keyword::Else),
      kw(Keyword::Skip),
      kw(Keyword::End),
      kw(Keyword::End),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::While(
        BExpression::True,
        Box::new(Statement::If(
          BExpression::True,
          Box::new(Statement::Skip),
          Box::new(Statement::Skip),
        )),
      )
    );
  }

  #[test]
  fn parse_sequence_inside_while_body() {
    // while true do x := 1 ; y := 2 end
    let tokens = stream(vec![
      kw(Keyword::While),
      Token::Lit(Literal::True),
      kw(Keyword::Do),
      var("x"),
      op(Operator::Assign),
      int(1),
      op(Operator::Semicolon),
      var("y"),
      op(Operator::Assign),
      int(2),
      kw(Keyword::End),
    ]);
    let result = parse(&tokens).unwrap();
    assert_eq!(
      result,
      Statement::While(
        BExpression::True,
        Box::new(Statement::Sequence(
          Box::new(Statement::Assign("x".to_string(), AExpression::Int(1))),
          Box::new(Statement::Assign("y".to_string(), AExpression::Int(2))),
        )),
      )
    );
  }

  #[test]
  fn parse_factorial_program() {
    // x := 6 ; result := 1 ; while !(x <= 0) do result := result * x ; x := x - 1 end
    let tokens = stream(vec![
      var("x"),
      op(Operator::Assign),
      int(6),
      op(Operator::Semicolon),
      var("result"),
      op(Operator::Assign),
      int(1),
      op(Operator::Semicolon),
      kw(Keyword::While),
      op(Operator::Not),
      del(Delimiter::LParen),
      var("x"),
      op(Operator::Leq),
      int(0),
      del(Delimiter::RParen),
      kw(Keyword::Do),
      var("result"),
      op(Operator::Assign),
      var("result"),
      op(Operator::Mul),
      var("x"),
      op(Operator::Semicolon),
      var("x"),
      op(Operator::Assign),
      var("x"),
      op(Operator::Sub),
      int(1),
      kw(Keyword::End),
    ]);
    // Just assert it parses without error — structure verified by simpler tests above
    assert!(parse(&tokens).is_ok());
  }

  // -------------------------------------------------------------------------
  // parse — error paths
  // -------------------------------------------------------------------------

  #[test]
  fn parse_empty_returns_err() {
    assert!(parse(&[]).is_err());
  }

  #[test]
  fn parse_missing_then_returns_err() {
    // if true skip else skip end  — missing then
    let tokens = stream(vec![
      kw(Keyword::If),
      Token::Lit(Literal::True),
      kw(Keyword::Skip),
      kw(Keyword::Else),
      kw(Keyword::Skip),
      kw(Keyword::End),
    ]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_missing_else_returns_err() {
    // if true then skip end  — missing else
    let tokens = stream(vec![
      kw(Keyword::If),
      Token::Lit(Literal::True),
      kw(Keyword::Then),
      kw(Keyword::Skip),
      kw(Keyword::End),
    ]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_missing_end_on_if_returns_err() {
    // if true then skip else skip  — missing end
    let tokens = stream(vec![
      kw(Keyword::If),
      Token::Lit(Literal::True),
      kw(Keyword::Then),
      kw(Keyword::Skip),
      kw(Keyword::Else),
      kw(Keyword::Skip),
    ]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_missing_do_returns_err() {
    // while true skip end  — missing do
    let tokens = stream(vec![
      kw(Keyword::While),
      Token::Lit(Literal::True),
      kw(Keyword::Skip),
      kw(Keyword::End),
    ]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_missing_end_on_while_returns_err() {
    // while true do skip  — missing end
    let tokens = stream(vec![
      kw(Keyword::While),
      Token::Lit(Literal::True),
      kw(Keyword::Do),
      kw(Keyword::Skip),
    ]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_orphan_then_returns_err() {
    let tokens = stream(vec![kw(Keyword::Then)]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_orphan_else_returns_err() {
    let tokens = stream(vec![kw(Keyword::Else)]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_orphan_do_returns_err() {
    let tokens = stream(vec![kw(Keyword::Do)]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_orphan_end_returns_err() {
    let tokens = stream(vec![kw(Keyword::End)]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_assign_missing_rhs_returns_err() {
    // x :=   — nothing after assign
    let tokens = stream(vec![var("x"), op(Operator::Assign)]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_bare_ident_returns_err() {
    // x   — identifier with no :=
    let tokens = stream(vec![var("x")]);
    assert!(parse(&tokens).is_err());
  }

  #[test]
  fn parse_unconsumed_tokens_returns_err() {
    // skip skip  — two statements with no semicolon between them
    // second skip becomes unconsumed
    let tokens = stream(vec![kw(Keyword::Skip), kw(Keyword::Skip)]);
    assert!(parse(&tokens).is_err());
  }
}
