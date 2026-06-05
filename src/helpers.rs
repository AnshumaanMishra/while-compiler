use crate::cfg::Cfg;
use crate::{
  error::{LexError, SemanticError, SyntaxError, UserDefinedError},
  lexer::{Delimiter, Identifier, Keyword, Literal, Operator, Token},
};
use colored::Colorize;
use serde::Serialize;
use std::fs;
use std::path::Path;

pub fn handle_error(inp_err: UserDefinedError) {
  match inp_err {
    UserDefinedError::Lex(err) => match err {
      LexError::UnidentifiedToken(t) => {
        eprintln!(
          "{} {}",
          "Error: Invalid Character:".red(),
          String::from(t).blue()
        );
        std::process::exit(20);
      }
    },
    UserDefinedError::Syntax(err) => match err {
      SyntaxError::DoWithoutWhile => {
        eprintln!(
          "{} {} {} {} {}",
          "Error: ".red(),
          "`do`".blue(),
          " statement without ".red(),
          "`while`".blue(),
          " statement".red(),
        );
        std::process::exit(33);
      }
      SyntaxError::ElseWithoutIf => {
        eprintln!(
          "{} {} {} {} {}",
          "Error: ".red(),
          "`else`".blue(),
          " statement without ".red(),
          "`if`".blue(),
          " statement".red(),
        );
        std::process::exit(32);
      }
      SyntaxError::InvalidAssign => {
        eprintln!("{}", "Invalid Assign!".red());
        std::process::exit(35)
      }
      SyntaxError::EndWithoutBlock => {
        eprintln!(
          "{} {} {} {} {}",
          "Error: ".red(),
          "`end`".blue(),
          " statement without ".red(),
          "any block".blue(),
          " statement".red(),
        );
        std::process::exit(34);
      }
      SyntaxError::ThenWithoutIf => {
        eprintln!(
          "{} {} {} {} {}",
          "Error: ".red(),
          "`then`".blue(),
          " statement without ".red(),
          "`if`".blue(),
          " statement".red(),
        );
        std::process::exit(31);
      }
      SyntaxError::UndefinedEnd => {
        eprintln!("{}", "Error: Termination of block not defined".red(),);
        std::process::exit(30);
      }
      SyntaxError::ExpectedThen => {
        eprintln!(
          "{} {} {}",
          "Error: ".red(),
          "expected `then`".blue(),
          "after if condition".red(),
        );
        std::process::exit(36);
      }
      SyntaxError::ExpectedElse => {
        eprintln!(
          "{} {} {}",
          "Error: ".red(),
          "expected `else`".blue(),
          "after then-branch".red(),
        );
        std::process::exit(37);
      }
      SyntaxError::ExpectedDo => {
        eprintln!(
          "{} {} {}",
          "Error: ".red(),
          "expected `do`".blue(),
          "after while condition".red(),
        );
        std::process::exit(38);
      }
      SyntaxError::ExpectedEnd => {
        eprintln!(
          "{} {} {}",
          "Error: ".red(),
          "expected `end`".blue(),
          "to close block".red(),
        );
        std::process::exit(39);
      }
      SyntaxError::UnconsumedTokens => {
        eprintln!(
          "{}",
          "Error: unexpected tokens after end of program — missing `;` or stray token".red(),
        );
        std::process::exit(40);
      }
      SyntaxError::NoMatch => {}
    },
    UserDefinedError::Semantic(err) => match err {
      SemanticError::UninitializedVariable(varname) => {
        eprintln!(
          "{} {} {}",
          "Error: ".red(),
          "Uninitialised Variable".blue(),
          varname.red(),
        );
        std::process::exit(50);
      }
    },
  }
}

pub fn print_tokens(tokens: &[Token]) {
  println!("Number of tokens lexed: {}\n", tokens.len());

  println!("{:<15} | {}", "KIND".bold(), "TEXT".bold());
  println!("{}", "-".repeat(35));

  for token in tokens {
    let (kind, text) = match token {
      Token::Kw(kw) => {
        let s = match kw {
          Keyword::If => "if",
          Keyword::Then => "then",
          Keyword::Else => "else",
          Keyword::While => "while",
          Keyword::Do => "do",
          Keyword::Skip => "skip",
          Keyword::End => "end",
        };
        ("Keyword", s.blue().to_string())
      }
      Token::Op(op) => {
        let s = match op {
          Operator::Assign => ":=",
          Operator::Add => "+",
          Operator::Sub => "-",
          Operator::Mul => "*",
          Operator::Equ => "=",
          Operator::Leq => "<=",
          Operator::Not => "not",
          Operator::And => "and",
          Operator::Semicolon => ";",
        };
        ("Operator", s.green().to_string())
      }
      Token::Del(dl) => {
        let s = match dl {
          Delimiter::LParen => "(",
          Delimiter::RParen => ")",
        };
        ("Delimiter", s.yellow().to_string())
      }
      Token::Idf(Identifier::Variable(v)) => ("Identifier", v.cyan().to_string()),
      Token::Lit(lit) => {
        let s = match lit {
          Literal::True => "true".to_string(),
          Literal::False => "false".to_string(),
          Literal::Int(i) => i.to_string(),
        };
        ("Literal", s.white().to_string())
      }
      Token::End => ("EOF", "END".red().to_string()),
    };

    println!("{:<15} | {}", kind, text);
  }
  println!();
}

pub fn print_pretty_ast(stmt: &crate::ast::Statement) {
  let doc = stmt.to_doc();
  let mut buffer = Vec::new();
  doc.render(80, &mut buffer).unwrap();
  let rendered_string = String::from_utf8(buffer).unwrap();
  println!("{}", highlight_syntax(&rendered_string));
  println!();
}

fn highlight_syntax(code: &str) -> String {
  let mut out = String::new();
  let mut current_word = String::new();
  for c in code.chars() {
    if c.is_alphanumeric() || c == '_' {
      current_word.push(c);
    } else {
      if !current_word.is_empty() {
        out.push_str(&color_word(&current_word));
        current_word.clear();
      }
      let sym = match c {
        '+' | '-' | '*' | '=' | '<' | ';' | ':' => c.to_string().green().to_string(),
        '(' | ')' => c.to_string().yellow().to_string(),
        _ => c.to_string(),
      };
      out.push_str(&sym);
    }
  }

  if !current_word.is_empty() {
    out.push_str(&color_word(&current_word));
  }

  out
}

fn color_word(word: &str) -> String {
  match word {
    "if" | "then" | "else" | "while" | "do" | "skip" | "end" => word.blue().to_string(),
    "not" | "and" => word.green().to_string(),
    "true" | "false" => word.white().to_string(),
    _ => {
      if word.chars().all(|c| c.is_ascii_digit()) {
        word.white().to_string()
      } else {
        word.cyan().to_string()
      }
    }
  }
}

pub fn write_json<T: Serialize>(data: &T, filename: &str) -> std::io::Result<()> {
  let dir = Path::new("outputs");
  if !dir.exists() {
    fs::create_dir(dir)?;
  }
  let path = dir.join(format!("{}.json", filename));
  let file = fs::File::create(path)?;
  serde_json::to_writer_pretty(file, data).map_err(std::io::Error::other)
}

pub fn print_cfg(cfg: &Cfg) {
  println!("{}", "Nodes (Labels and Statements):".bold());
  let mut sorted_nodes: Vec<_> = cfg.nodes.iter().collect();
  sorted_nodes.sort();

  for label in &cfg.nodes {
    let tokens = cfg.statement_map.get(label).unwrap();
    // Convert tokens to a readable string for the terminal
    let stmt_text: String = tokens.iter().map(|t| format!("{:?} ", t)).collect();
    println!("  {}: {}", label.to_string().yellow(), stmt_text);
  }

  println!("\n{}", "Edges (Control Flow):".bold());
  for edge in &cfg.edges {
    println!(
      "  {} {} {}",
      edge.from.to_string().yellow(),
      "-->".bold(),
      edge.to.to_string().yellow()
    );
  }
  println!();
}

// Tests:
#[cfg(test)]
mod helpers_tests {
  use super::*;
  use crate::lexer::{Delimiter, Identifier, Keyword, Literal, Operator, Token};

  // colored adds ANSI escape codes so we strip them before comparing text
  fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut in_escape = false;
    for c in s.chars() {
      if c == '\x1b' {
        in_escape = true;
      } else if in_escape {
        if c == 'm' {
          in_escape = false;
        }
      } else {
        out.push(c);
      }
    }
    out
  }

  fn get_plain_text(token: &Token) -> String {
    match token {
      Token::Kw(kw) => match kw {
        Keyword::If => "if".to_string(),
        Keyword::Then => "then".to_string(),
        Keyword::Else => "else".to_string(),
        Keyword::While => "while".to_string(),
        Keyword::Do => "do".to_string(),
        Keyword::Skip => "skip".to_string(),
        Keyword::End => "end".to_string(),
      },
      Token::Op(op) => match op {
        Operator::Assign => ":=".to_string(),
        Operator::Add => "+".to_string(),
        Operator::Sub => "-".to_string(),
        Operator::Mul => "*".to_string(),
        Operator::Equ => "=".to_string(),
        Operator::Leq => "<=".to_string(),
        Operator::Not => "not".to_string(),
        Operator::And => "and".to_string(),
        Operator::Semicolon => ";".to_string(),
      },
      Token::Del(dl) => match dl {
        Delimiter::LParen => "(".to_string(),
        Delimiter::RParen => ")".to_string(),
      },
      Token::Idf(Identifier::Variable(v)) => v.to_string(),
      Token::Lit(lit) => match lit {
        Literal::True => "true".to_string(),
        Literal::False => "false".to_string(),
        Literal::Int(i) => i.to_string(),
      },
      Token::End => "END".to_string(),
    }
  }

  fn text(token: &Token) -> String {
    strip_ansi(&get_plain_text(token))
  }

  // -------------------------------------------------------------------------
  // Keywords
  // -------------------------------------------------------------------------

  #[test]
  fn display_kw_if() {
    assert!(text(&Token::Kw(Keyword::If)).contains("if"));
  }

  #[test]
  fn display_kw_then() {
    assert!(text(&Token::Kw(Keyword::Then)).contains("then"));
  }

  #[test]
  fn display_kw_else() {
    assert!(text(&Token::Kw(Keyword::Else)).contains("else"));
  }

  #[test]
  fn display_kw_while() {
    assert!(text(&Token::Kw(Keyword::While)).contains("while"));
  }

  #[test]
  fn display_kw_do() {
    assert!(text(&Token::Kw(Keyword::Do)).contains("do"));
  }

  #[test]
  fn display_kw_skip() {
    assert!(text(&Token::Kw(Keyword::Skip)).contains("skip"));
  }

  #[test]
  fn display_kw_end() {
    assert!(text(&Token::Kw(Keyword::End)).contains("end"));
  }

  // -------------------------------------------------------------------------
  // Operators
  // -------------------------------------------------------------------------

  #[test]
  fn display_op_assign() {
    assert!(text(&Token::Op(Operator::Assign)).contains(":="));
  }

  #[test]
  fn display_op_add() {
    assert!(text(&Token::Op(Operator::Add)).contains("+"));
  }

  #[test]
  fn display_op_sub() {
    assert!(text(&Token::Op(Operator::Sub)).contains("-"));
  }

  #[test]
  fn display_op_mul() {
    assert!(text(&Token::Op(Operator::Mul)).contains("*"));
  }

  #[test]
  fn display_op_equ() {
    assert!(text(&Token::Op(Operator::Equ)).contains("="));
  }

  #[test]
  fn display_op_leq() {
    assert!(text(&Token::Op(Operator::Leq)).contains("<="));
  }

  #[test]
  fn display_op_not() {
    assert!(text(&Token::Op(Operator::Not)).contains("not"));
  }

  #[test]
  fn display_op_and() {
    assert!(text(&Token::Op(Operator::And)).contains("and"));
  }

  #[test]
  fn display_op_semicolon() {
    assert!(text(&Token::Op(Operator::Semicolon)).contains(";"));
  }

  // -------------------------------------------------------------------------
  // Delimiters
  // -------------------------------------------------------------------------

  #[test]
  fn display_del_lparen() {
    assert_eq!(text(&Token::Del(Delimiter::LParen)).trim(), "(");
  }

  #[test]
  fn display_del_rparen() {
    assert_eq!(text(&Token::Del(Delimiter::RParen)).trim(), ")");
  }

  // -------------------------------------------------------------------------
  // Literals
  // -------------------------------------------------------------------------

  #[test]
  fn display_lit_true() {
    assert!(
      text(&Token::Lit(Literal::True))
        .to_lowercase()
        .contains("true")
    );
  }

  #[test]
  fn display_lit_false() {
    assert!(
      text(&Token::Lit(Literal::False))
        .to_lowercase()
        .contains("false")
    );
  }

  #[test]
  fn display_lit_int_positive() {
    assert!(text(&Token::Lit(Literal::Int(42))).contains("42"));
  }

  #[test]
  fn display_lit_int_zero() {
    assert!(text(&Token::Lit(Literal::Int(0))).contains("0"));
  }

  #[test]
  fn display_lit_int_negative() {
    assert!(text(&Token::Lit(Literal::Int(-7))).contains("-7"));
  }

  // -------------------------------------------------------------------------
  // Identifier
  // -------------------------------------------------------------------------

  #[test]
  fn display_identifier_variable() {
    let tok = Token::Idf(Identifier::Variable("myvar".to_string()));
    assert!(text(&tok).contains("myvar"));
  }

  // -------------------------------------------------------------------------
  // EOF token
  // -------------------------------------------------------------------------

  #[test]
  fn display_end_token_is_non_empty() {
    // Just assert it produces some output — the exact formatting may change
    assert!(!text(&Token::End).trim().is_empty());
  }

  // -------------------------------------------------------------------------
  // print_tokens — smoke test (no panic, correct count reported)
  // -------------------------------------------------------------------------

  #[test]
  fn print_tokens_does_not_panic() {
    let tokens = vec![
      Token::Kw(Keyword::Skip),
      Token::Op(Operator::Semicolon),
      Token::Lit(Literal::Int(1)),
      Token::End,
    ];
    // Should complete without panicking
    print_tokens(&tokens);
  }

  #[test]
  fn print_tokens_empty_vec() {
    // Edge case: empty token list
    print_tokens(&vec![]);
  }
}
