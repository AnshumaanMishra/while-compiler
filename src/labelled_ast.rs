use crate::ast::{AExpression, BExpression};
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

impl fmt::Display for StatementL {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StatementL::Assign(l, var, expr) => write!(f, "[{} := {}]{}", var, expr, l),
      StatementL::Skip(l) => write!(f, "[skip]{}", l),
      StatementL::Sequence(s1, s2) => write!(f, "{}; {}", s1, s2),
      StatementL::If(l, b, _, _) => write!(f, "[{}]{}", b, l),
      StatementL::While(l, b, _) => write!(f, "[{}]{}", b, l),
    }
  }
}
