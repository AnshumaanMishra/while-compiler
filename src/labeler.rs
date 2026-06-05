use crate::ast::Statement as Stmt;
use crate::labelled_ast::{Label, StatementL as StmtL};

pub fn label_ast(stmt: Stmt, counter: &mut Label) -> StmtL {
  match &stmt {
    Stmt::Assign(v, e) => {
      let label = *counter;
      *counter += 1;
      StmtL::Assign(label, v.clone(), e.clone())
    }
    Stmt::Skip => {
      let label = *counter;
      *counter += 1;
      StmtL::Skip(label)
    }
    Stmt::Sequence(s1, s2) => StmtL::Sequence(
      Box::new(label_ast(*s1.clone(), counter)),
      Box::new(label_ast(*s2.clone(), counter)),
    ),
    Stmt::If(b, s1, s2) => {
      let label = *counter;
      *counter += 1;
      StmtL::If(
        label,
        b.clone(),
        Box::new(label_ast(*s1.clone(), counter)),
        Box::new(label_ast(*s2.clone(), counter)),
      )
    }
    Stmt::While(b, s) => {
      let label = *counter;
      *counter += 1;
      StmtL::While(label, b.clone(), Box::new(label_ast(*s.clone(), counter)))
    }
  }
}
