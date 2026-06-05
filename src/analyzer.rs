use crate::{
  ast::{AExpression, BExpression, Statement},
  error::UserDefinedError,
};
use std::collections::HashSet;

use crate::error::SemanticError;

pub struct SemanticAnalyzer {
  initialized_vars: HashSet<String>,
}

impl SemanticAnalyzer {
  pub fn new() -> Self {
    Self {
      initialized_vars: HashSet::new(),
    }
  }

  pub fn visit_stmt(&mut self, stmt: &Statement) -> Result<(), UserDefinedError> {
    match stmt {
      Statement::Assign(var, expr) => {
        self.visit_aexpr(expr)?;

        self.initialized_vars.insert(var.clone());
        Ok(())
      }
      Statement::Sequence(stmt1, stmt2) => {
        self.visit_stmt(stmt1)?;
        self.visit_stmt(stmt2)
      }
      Statement::If(condition, stmt1, stmt2) => {
        self.visit_bexpr(condition)?;
        let state_before = self.initialized_vars.clone();

        self.visit_stmt(stmt1)?;
        let state_after_then = self.initialized_vars.clone();

        self.initialized_vars = state_before;
        self.visit_stmt(stmt2)?;
        let state_after_else = self.initialized_vars.clone();

        self.initialized_vars = state_after_then
          .intersection(&state_after_else)
          .cloned()
          .collect();
        Ok(())
      }
      Statement::While(condition, stmt) => {
        self.visit_bexpr(condition)?;
        self.visit_stmt(stmt)
      }
      Statement::Skip => Ok(()),
    }
  }

  pub fn visit_bexpr(&mut self, bexpr: &BExpression) -> Result<(), UserDefinedError> {
    match bexpr {
      BExpression::True => Ok(()),
      BExpression::False => Ok(()),
      BExpression::And(left, right) => {
        self.visit_bexpr(left)?;
        self.visit_bexpr(right)
      }
      BExpression::Equ(left, right) | BExpression::Leq(left, right) => {
        self.visit_aexpr(left)?;
        self.visit_aexpr(right)
      }
      BExpression::Not(bxpr) => self.visit_bexpr(bxpr),
    }
  }

  pub fn visit_aexpr(&mut self, aexpr: &AExpression) -> Result<(), UserDefinedError> {
    match aexpr {
      AExpression::Int(_) => Ok(()),
      AExpression::Add(stmt1, stmt2)
      | AExpression::Sub(stmt1, stmt2)
      | AExpression::Mul(stmt1, stmt2) => {
        self.visit_aexpr(stmt1)?;
        self.visit_aexpr(stmt2)
      }
      AExpression::Var(var) => {
        if self.initialized_vars.contains(var) {
          Ok(())
        } else {
          Err(UserDefinedError::Semantic(
            SemanticError::UninitializedVariable(var.clone()),
          ))
        }
      }
    }
  }
}
