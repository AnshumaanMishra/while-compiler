use crate::labelled_ast::{Label, StatementL};
use crate::lexer::Token;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Edge {
  pub from: Label,
  pub to: Label,
}

#[derive(Debug, Serialize)]
pub struct Cfg {
  pub nodes: Vec<Label>,
  pub edges: Vec<Edge>,
  pub statement_map: HashMap<Label, Vec<Token>>,
}

impl Cfg {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new(),
      edges: Vec::new(),
      statement_map: HashMap::new(),
    }
  }

  pub fn add_node(&mut self, label: Label, stmt: &StatementL) {
    if !self.nodes.contains(&label) {
      self.nodes.push(label);
      self.statement_map.insert(label, stmt.to_tokens());
    }
  }

  pub fn add_edge(&mut self, from: Label, to: Label) {
    if !self.edges.iter().any(|e| e.from == from && e.to == to) {
      self.edges.push(Edge { from, to });
    }
  }
}

pub fn build_cfg(stmt: &StatementL, cfg: &mut Cfg) -> (Label, Vec<Label>) {
  match stmt {
    StatementL::Assign(l, _, _) => {
      cfg.add_node(*l, stmt);
      (*l, vec![*l])
    }
    StatementL::Skip(l) => {
      cfg.add_node(*l, stmt);
      (*l, vec![*l])
    }

    StatementL::Sequence(s1, s2) => {
      let (entry1, exits1) = build_cfg(s1, cfg);
      let (entry2, exits2) = build_cfg(s2, cfg);
      for e in &exits1 {
        cfg.add_edge(*e, entry2);
      }
      (entry1, exits2)
    }

    StatementL::If(l, _, s1, s2) => {
      cfg.add_node(*l, stmt);
      let (entry1, exits1) = build_cfg(s1, cfg);
      let (entry2, exits2) = build_cfg(s2, cfg);
      cfg.add_edge(*l, entry1);
      cfg.add_edge(*l, entry2);
      (*l, exits1.into_iter().chain(exits2).collect())
    }

    StatementL::While(l, _, body) => {
      cfg.add_node(*l, stmt);
      let (entry_body, exits_body) = build_cfg(body, cfg);
      cfg.add_edge(*l, entry_body);
      for e in &exits_body {
        cfg.add_edge(*e, *l);
      }
      (*l, vec![*l])
    }
  }
}
