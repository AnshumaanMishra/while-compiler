use crate::ast::{AExpression, BExpression, Statement};
use std::collections::HashMap;

pub type Store = HashMap<String, i64>;

pub fn eval_aexpr(expr: &AExpression, store: &Store) -> i64 {
  match expr {
    AExpression::Var(varname) => *store
      .get(varname)
      .expect("Variable not found in store, semantic analysis failed"),
    AExpression::Int(value) => *value,
    AExpression::Add(left, right) => eval_aexpr(left, store) + eval_aexpr(right, store),
    AExpression::Sub(left, right) => eval_aexpr(left, store) - eval_aexpr(right, store),
    AExpression::Mul(left, right) => eval_aexpr(left, store) * eval_aexpr(right, store),
  }
}

pub fn eval_bexpr(expr: &BExpression, store: &Store) -> bool {
  match expr {
    BExpression::True => true,
    BExpression::False => false,
    BExpression::And(left, right) => eval_bexpr(left, store) && eval_bexpr(right, store),
    BExpression::Equ(left, right) => eval_aexpr(left, store) == eval_aexpr(right, store),
    BExpression::Leq(left, right) => eval_aexpr(left, store) <= eval_aexpr(right, store),
    BExpression::Not(bxpr) => !(eval_bexpr(bxpr, store)),
  }
}

pub fn exec_stmt(stmt: &Statement, store: &mut Store) {
  match stmt {
    Statement::Skip => {}
    Statement::Assign(var, expr) => {
      let value = eval_aexpr(expr, store);
      store.insert(var.clone(), value);
    }
    Statement::Sequence(stmt1, stmt2) => {
      exec_stmt(stmt1, store);
      exec_stmt(stmt2, store);
    }
    Statement::If(cond, then_branch, else_branch) => {
      if eval_bexpr(cond, store) {
        exec_stmt(then_branch, store);
      } else {
        exec_stmt(else_branch, store);
      }
    }
    Statement::While(cond, body) => {
      while eval_bexpr(cond, store) {
        exec_stmt(body, store);
      }
    }
  }
}
