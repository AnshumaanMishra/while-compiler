mod ast;
mod error;
mod helpers;
mod lexer;

use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

// use crate::ast::{AExpression, BExpression, Statement};
use crate::ast::parse;
use crate::error::{FileError, UserDefinedError};
use crate::helpers::{handle_error, print_syntax_tree};
use crate::lexer::{Token, parse_tokens};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(
    short = 'i',
    long = "input",
    value_name = "FILE",
    help = "Path to the source file"
  )]
  input: Option<PathBuf>,

  #[arg(
    short = 'o',
    long = "output",
    value_name = "FILE",
    help = "Path to output"
  )]
  output: Option<PathBuf>,

  // The reason why there are 3 arguments, is because clap does not allow both
  // flag indicators and sequential input. Eg. If I only did the two above this line,
  // I would be able to pass as `binary -i ./input.wl -o output` but not as
  // `binary input.wl output`
  // The reverse would be true if I only used the option below this.
  #[arg(value_name = "FILES", help = "Input file followed by output file")]
  files: Vec<PathBuf>,
}

// This function manually parses the arguments list to allot input and output file directories based on priority.
fn parse_arguments(args: &Args) -> Result<(PathBuf, PathBuf), UserDefinedError> {
  // Resolve the input path first
  let input_path = if let Some(path) = &args.input {
    path.clone()
  } else if let Some(path) = args.files.first() {
    path.clone()
  } else {
    return Err(UserDefinedError::File(FileError::InputArgumentEmpty));
  };

  // Resolve the output path independently
  let output_path = if let Some(path) = &args.output {
    path.clone()
  } else if let Some(path) = args.files.get(1) {
    path.clone()
  } else {
    PathBuf::from("./out")
  };

  Ok((input_path, output_path))
}

fn main() {
  let args = Args::parse();
  let input_file: PathBuf;
  let output_file: PathBuf;
  match parse_arguments(&args) {
    Ok((a, b)) => {
      input_file = a;
      output_file = b;
    }
    Err(s) => {
      handle_error(s);
      exit(1);
    }
  }
  println!(
    "Attempting to read from {} and output to {}",
    input_file.display().to_string().blue(),
    output_file.display().to_string().blue()
  );

  match fs::read_to_string(&input_file) {
    Err(e) => {
      handle_error(UserDefinedError::File(FileError::BuiltinError((
        String::from(input_file.to_str().unwrap()),
        e,
      ))));
    }
    Ok(contents) => {
      println!("{}", contents);
      let exploded_source: Vec<char> = contents.chars().collect();
      let mut tokens: Vec<Token> = Vec::new();
      match parse_tokens(&exploded_source, &mut tokens) {
        Ok(()) => match parse(&tokens) {
          Ok(ast) => print_syntax_tree(&ast, 2),
          Err(e) => handle_error(e),
        },
        Err(e) => handle_error(e),
      }
    }
  }
}
