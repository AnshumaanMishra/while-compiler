mod ast;
mod lexer;

use crate::ast::{AExpression, BExpression, Statement};
// use colored::Colorize;

fn main() {
  println!(
    "{}",
    AExpression::Add(Box::new(AExpression::Int(5)), Box::new(AExpression::Int(5)))
  );
  println!("{}", BExpression::True);
  println!(
    "{}",
    Statement::If(
      BExpression::True,
      Box::new(Statement::Skip),
      Box::new(Statement::Skip)
    )
  );
}
