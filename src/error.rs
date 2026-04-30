use std::io::Error;

#[derive(Debug)]
pub enum UserDefinedError {
  FileErr(FileError),
  LexErr(LexError),
}

#[derive(Debug)]
pub enum FileError {
  InputArgumentEmpty,
  BuiltinError((String, Error)),
}

#[derive(Debug)]
pub enum LexError {
  UnidentifiedToken(char),
}
