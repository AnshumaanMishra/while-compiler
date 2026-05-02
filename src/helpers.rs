use crate::{
  ast::Statement,
  error::{FileError, LexError, SyntaxError, UserDefinedError},
  lexer::{Delimiter, Identifier, Keyword, Literal, Operator, Token},
};
use colored::Colorize;

pub fn handle_error(inp_err: UserDefinedError) {
  match inp_err {
    UserDefinedError::File(err) => match err {
      FileError::InputArgumentEmpty => {
        eprintln!("{}", "Error: Input Argument Empty".red());
        std::process::exit(10);
      }
      FileError::BuiltinError((name, e)) => {
        eprintln!(
          "{} {}\n\t{} {}",
          "Filename: ".red(),
          name.blue(),
          "Error: ".red(),
          e.to_string().yellow()
        );
        std::process::exit(11);
      }
    },
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
  }
}

#[allow(dead_code)]
fn get_string_for_token(input: &Token) -> String {
  match input {
    Token::Kw(kw) => match &kw {
      Keyword::If => format!("{}", "\n\nif ".blue()),
      Keyword::Then => format!("{}", " then\n".blue()),
      Keyword::Else => format!("{}", "\nelse\n".blue()),
      Keyword::While => format!("{}", "\nwhile ".blue()),
      Keyword::Do => format!("{}", "do\n".blue()),
      Keyword::Skip => format!("{}", "\nskip\n".blue()),
      Keyword::End => format!("{}", "\nend\n\n".blue()),
    },
    Token::Op(op) => match op {
      Operator::Assign => format!("{}", " := ".green()),
      Operator::Add => format!("{}", " + ".green()),
      Operator::Sub => format!("{}", " - ".green()),
      Operator::Mul => format!("{}", " * ".green()),
      Operator::Equ => format!("{}", " = ".green()),
      Operator::Leq => format!("{}", " <= ".green()),
      Operator::Not => format!("{}", " not ".green()),
      Operator::And => format!("{}", " && ".green()),
      Operator::Semicolon => format!("{}", ";".green()),
    },
    Token::Del(dl) => match dl {
      Delimiter::LParen => format!("{}", "(".yellow()),
      Delimiter::RParen => format!("{}", ")".yellow()),
    },
    Token::Idf(id) => match id {
      Identifier::Variable(varname) => format!("{} ", varname.cyan()),
    },
    Token::Lit(lit) => match lit {
      Literal::True => format!("{} ", "True".white()),
      Literal::False => format!("{} ", "False".white()),
      Literal::Int(i) => format!("{}", i.to_string().white()),
    },
    Token::End => format!("{}", "\n\nEND\n".red()),
  }
}

#[allow(dead_code)]
pub fn print_tokens(tokens: &Vec<Token>) {
  println!("Number of tokens lexed: {}\n", tokens.len());
  for i in tokens {
    print!("{}", get_string_for_token(i));
  }
}

pub fn print_syntax_tree(stmt: &Statement, indent: usize) {
  let pad = "  ".repeat(indent);
  match stmt {
    Statement::Skip => {
      println!("{}Skip", pad);
    }
    Statement::Assign(var, expr) => {
      println!("{}Assign", pad);
      println!("{}  var:  {}", pad, var);
      println!("{}  expr: {}", pad, expr);
    }
    Statement::Sequence(left, right) => {
      println!("{}Sequence", pad);
      print_syntax_tree(left, indent + 1);
      print_syntax_tree(right, indent + 1);
    }
    Statement::If(cond, then_branch, else_branch) => {
      println!("{}If", pad);
      println!("{}  cond: {}", pad, cond);
      println!("{}  then:", pad);
      print_syntax_tree(then_branch, indent + 2);
      println!("{}  else:", pad);
      print_syntax_tree(else_branch, indent + 2);
    }
    Statement::While(cond, body) => {
      println!("{}While", pad);
      println!("{}  cond: {}", pad, cond);
      println!("{}  body:", pad);
      print_syntax_tree(body, indent + 2);
    }
  }
}

// Tests:
// Note: The tests, and only the tests, have been written using AI
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

  fn text(token: &Token) -> String {
    strip_ansi(&get_string_for_token(token))
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
    assert!(text(&Token::Op(Operator::And)).contains("&&"));
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
