use crate::ast::{AExpression, BExpression};
use crate::lexer::{Identifier, Keyword, Operator, Token};
use serde::Serialize;
use std::fmt;

pub type Label = usize;

#[derive(Debug, Serialize, Clone)]
pub enum StatementL {
  Assign(Label, String, AExpression),
  Sequence(Box<StatementL>, Box<StatementL>),
  If(Label, BExpression, Box<StatementL>, Box<StatementL>),
  While(Label, BExpression, Box<StatementL>),
  Skip(Label),
}
impl StatementL {
  pub fn to_tokens(&self) -> Vec<Token> {
    match self {
      StatementL::Assign(_, var, expr) => {
        let mut tokens = vec![
          Token::Idf(Identifier::Variable(var.clone())),
          Token::Op(Operator::Assign),
        ];
        tokens.extend(expr.to_tokens());
        tokens
      }
      StatementL::Skip(_) => vec![Token::Kw(Keyword::Skip)],
      StatementL::Sequence(s1, s2) => {
        let mut tokens = s1.to_tokens();
        tokens.push(Token::Op(Operator::Semicolon));
        tokens.extend(s2.to_tokens());
        tokens
      }
      StatementL::If(_, b, s1, s2) => {
        let mut tokens = vec![Token::Kw(Keyword::If)];
        tokens.extend(b.to_tokens());
        tokens.push(Token::Kw(Keyword::Then));
        tokens.extend(s1.to_tokens());
        tokens.push(Token::Kw(Keyword::Else));
        tokens.extend(s2.to_tokens());
        tokens.push(Token::Kw(Keyword::End));
        tokens
      }
      StatementL::While(_, b, body) => {
        let mut tokens = vec![Token::Kw(Keyword::While)];
        tokens.extend(b.to_tokens());
        tokens.push(Token::Kw(Keyword::Do));
        tokens.extend(body.to_tokens());
        tokens.push(Token::Kw(Keyword::End));
        tokens
      }
    }
  }
}

impl fmt::Display for StatementL {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StatementL::Assign(l, var, expr) => write!(f, "[{} := {}]ˡ{}", var, expr, l),
      StatementL::Skip(l) => write!(f, "[skip]ˡ{}", l),
      StatementL::Sequence(s1, s2) => write!(f, "{}; {}", s1, s2),
      StatementL::If(l, b, _, _) => write!(f, "[if {}]ˡ{}", b, l),
      StatementL::While(l, b, _) => write!(f, "[while {}]ˡ{}", b, l),
    }
  }
}
