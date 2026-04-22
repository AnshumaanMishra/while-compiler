#![allow(dead_code)]

// Keywords Enum
pub enum Keyword {
  // If Block
  If,
  Then,
  Else,
  // While Loop
  While,
  Do,
  // Skip
  Skip,
  // Block Keyword
  End,
}

// Operators Enum
pub enum Operator {
  Assign,
  Add,
  Sub,
  Mul,
  Equ,
  Leq,
  Not,
  And,
  Semicolon,
}

// Delimiters Enum
pub enum Delimiter {
  LParen,
  RParen,
}

// Identifiers Enum
pub enum Identifier {
  Variable(String),
}

// Literals Enum
pub enum Literal {
  True,
  False,
  Int(i64),
}

// Token Enum
pub enum Token {
  // Main Keywords
  Keyword(Keyword),
  // Operators
  Operator(Operator),
  // Delimiter
  Delimiter(Delimiter),
  // Variable
  Identifier(Identifier),
  // Literals
  Literal(Literal),
  // EOF
  End,
}
