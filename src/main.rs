mod analyzer;
mod ast;
mod error;
mod helpers;
mod interpreter;
mod lexer;
// mod parser;

use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

// use crate::ast::{AExpression, BExpression, Statement};
use crate::analyzer::SemanticAnalyzer;
use crate::ast::parse;
use crate::helpers::{handle_error, print_pretty_ast, print_tokens, write_json};
use crate::interpreter::exec_stmt;
use crate::lexer::{Token, parse_tokens};

#[derive(Parser, Debug)]
#[command(author, version, about = "A compiler and interpreter for the While language", long_about = None)]
pub struct Cli {
  /// Path to the .while source file
  pub file: PathBuf,

  /// Lex and print the token stream, then exit
  #[arg(long)]
  pub tokens: bool,

  /// Parse and pretty-print the AST, then exit
  #[arg(long)]
  pub ast: bool,

  /// Execute the program and print the final state
  #[arg(long)]
  pub run: bool,
}

fn main() {
  let cli = Cli::parse();

  let source = match fs::read_to_string(&cli.file) {
    Ok(content) => content,
    Err(e) => {
      eprintln!(
        "{} {} {} {}",
        "Error reading file '".red(),
        cli.file.display().to_string().blue(),
        "': ".red(),
        e.to_string().blue()
      );
      std::process::exit(1);
    }
  };

  let chars: Vec<char> = source.chars().collect();

  let mut tokens: Vec<Token> = Vec::new();
  if let Err(e) = parse_tokens(&chars, &mut tokens) {
    handle_error(e);
    exit(1);
  }

  if cli.tokens {
    println!("{}", "Token Stream: ".blue());
    print_tokens(&tokens);
    write_json(&tokens, "tokens").expect("Failed to write tokens.json");
    return;
  }

  // 3. Parsing (Phase 4)
  let ast = match parse(&tokens) {
    Ok(tree) => tree,
    Err(e) => {
      handle_error(e);
      exit(1);
    }
  };

  if cli.ast {
    println!("{}", "Abstract Syntax Tree: ".blue());
    print_pretty_ast(&ast);
    write_json(&ast, "ast").expect("Failed to write ast.json");
    return;
  }

  let mut analyzer = SemanticAnalyzer::new();
  if let Err(e) = analyzer.visit_stmt(&ast) {
    eprintln!("Semantic Error: {:?}", e);
    exit(1);
  }

  if cli.run {
    println!("{}", "Executing Program: ".blue());
    let mut store = HashMap::new();
    exec_stmt(&ast, &mut store);

    println!("\nFinal Memory Store:");
    for (var, value) in &store {
      println!("  {} = {}", var, value);
    }
    write_json(&store, "store").expect("Failed to write store.json");
  } else {
    println!("Compilation and Semantic Analysis successful! Use --run to execute.");
  }
}
