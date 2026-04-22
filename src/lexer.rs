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

// EOF Enum
pub enum End {
  End,
}

// Token Enum
pub enum Token {
  // Main Keywords
  KeywordToken(Keyword),
  // Operators
  OperatorToken(Operator),
  // Delimiter
  DelimiterToken(Delimiter),
  // Variable
  IdentifierToken(Identifier),
  // Literals
  LiteralToken(Literal),
  // EOF
  EOFToken(End),
}
