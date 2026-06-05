// use std::io::Error;

#[derive(Debug)]
pub enum UserDefinedError {
  // File(FileError),
  Lex(LexError),
  Syntax(SyntaxError),
  Semantic(SemanticError),
}

// #[derive(Debug)]
// pub enum FileError {
//   InputArgumentEmpty,
//   BuiltinError((String, Error)),
// }

#[derive(Debug)]
pub enum LexError {
  UnidentifiedToken(char),
}

#[derive(Debug)]
pub enum SyntaxError {
  ThenWithoutIf,
  ElseWithoutIf,
  DoWithoutWhile,
  EndWithoutBlock,
  UndefinedEnd,
  NoMatch,
  InvalidAssign,
  ExpectedThen,
  ExpectedElse,
  ExpectedDo,
  ExpectedEnd,
  UnconsumedTokens,
}

#[derive(Debug)]
pub enum SemanticError {
  UninitializedVariable(String),
}
