#![allow(dead_code)]

use crate::error::{LexError, UserDefinedError};

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
  Kw(Keyword),
  // Operators
  Op(Operator),
  // Delimiter
  Del(Delimiter),
  // Variable
  Idf(Identifier),
  // Literals
  Lit(Literal),
  // EOF
  End,
}

// Lexes Integers. Negative integers are treated as integers with the unary minus
// operated on them
fn lex_int(chars: &[char]) -> (i64, usize) {
  let mut skip_len: usize = 0;
  let mut int: i64 = 0;
  while skip_len < chars.len() && chars[skip_len].is_ascii_digit() {
    int = int * 10 + (chars[skip_len].to_digit(10).unwrap() as i64);
    skip_len += 1;
  }
  (int, skip_len)
}

// Lexes Identifiers.
fn lex_idef(chars: &[char]) -> (String, usize) {
  let mut skip_len: usize = 0;
  let mut var_name = String::new();

  while chars[skip_len].is_alphanumeric() || chars[skip_len] == '_' {
    var_name.push(chars[skip_len]);
    skip_len += 1;
  }

  (var_name, skip_len)
}

pub fn parse_tokens(chars: &[char], tokens: &mut Vec<Token>) -> Result<(), UserDefinedError> {
  let skip_length = match chars {
    [] => 1,
    [':', '=', ..] => {
      tokens.push(Token::Op(Operator::Assign));
      2
    }
    ['<', '=', ..] => {
      tokens.push(Token::Op(Operator::Leq));
      2
    }
    ['=', ..] => {
      tokens.push(Token::Op(Operator::Equ));
      1
    }
    ['+', ..] => {
      tokens.push(Token::Op(Operator::Add));
      1
    }
    ['-', ..] => {
      tokens.push(Token::Op(Operator::Sub));
      1
    }
    ['*', ..] => {
      tokens.push(Token::Op(Operator::Mul));
      1
    }
    ['!', ..] => {
      tokens.push(Token::Op(Operator::Not));
      1
    }
    ['&', ..] => {
      tokens.push(Token::Op(Operator::And));
      1
    }
    [';', ..] => {
      tokens.push(Token::Op(Operator::Semicolon));
      1
    }
    ['(', ..] => {
      tokens.push(Token::Del(Delimiter::LParen));
      1
    }
    [')', ..] => {
      tokens.push(Token::Del(Delimiter::RParen));
      1
    }
    ['i', 'f', ..] => {
      tokens.push(Token::Kw(Keyword::If));
      2
    }
    ['t', 'h', 'e', 'n', ..] => {
      tokens.push(Token::Kw(Keyword::Then));
      4
    }
    ['e', 'l', 's', 'e', ..] => {
      tokens.push(Token::Kw(Keyword::Else));
      4
    }
    ['w', 'h', 'i', 'l', 'e', ..] => {
      tokens.push(Token::Kw(Keyword::While));
      5
    }
    ['d', 'o', ..] => {
      tokens.push(Token::Kw(Keyword::Do));
      2
    }
    ['s', 'k', 'i', 'p', ..] => {
      tokens.push(Token::Kw(Keyword::Skip));
      4
    }
    ['e', 'n', 'd', ..] => {
      tokens.push(Token::Kw(Keyword::End));
      3
    }
    ['t', 'r', 'u', 'e', ..] => {
      tokens.push(Token::Lit(Literal::True));
      4
    }
    ['f', 'a', 'l', 's', 'e', ..] => {
      tokens.push(Token::Lit(Literal::False));
      5
    }
    ['0'..='9', ..] => {
      let (num, skip) = lex_int(chars);
      tokens.push(Token::Lit(Literal::Int(num)));
      skip
    }
    ['a'..='z', ..] | ['_', ..] | ['A'..='Z', ..] => {
      let (varname, skip) = lex_idef(chars);
      tokens.push(Token::Idf(Identifier::Variable(varname)));
      skip
    }
    ['\n', ..] => 1,
    ['\t', ..] => 1,
    ['\r', ..] => 1,
    [' ', ..] => 1,
    _ => {
      return Err(UserDefinedError::Lex(LexError::UnidentifiedToken(chars[0])));
    }
  };
  // Used for debugging, do not uncomment
  // print!(
  //   "Token Parsed: {}, New Tokens length: {}, Skip Length = {}\n",
  //   get_string_for_token(&tokens[tokens.len() - 1]),
  //   tokens.len(),
  //   skip_length
  // );
  if chars.len() > skip_length {
    parse_tokens(&chars[skip_length..], tokens)
  } else {
    tokens.push(Token::End);
    Ok(())
  }
}

// Tests:
// Note: The tests(and only the tests) are written using AI
#[cfg(test)]
mod lexer_tests {
  use super::*;
  use crate::error::{LexError, UserDefinedError};

  // -------------------------------------------------------------------------
  // Helpers
  // -------------------------------------------------------------------------

  fn lex(src: &str) -> Result<Vec<Token>, UserDefinedError> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    parse_tokens(&chars, &mut tokens)?;
    Ok(tokens)
  }

  // Extracts the discriminant name so tests don't need to match every field
  fn token_kind(t: &Token) -> &'static str {
    match t {
      Token::Kw(Keyword::If) => "If",
      Token::Kw(Keyword::Then) => "Then",
      Token::Kw(Keyword::Else) => "Else",
      Token::Kw(Keyword::While) => "While",
      Token::Kw(Keyword::Do) => "Do",
      Token::Kw(Keyword::Skip) => "Skip",
      Token::Kw(Keyword::End) => "End",
      Token::Op(Operator::Assign) => "Assign",
      Token::Op(Operator::Add) => "Add",
      Token::Op(Operator::Sub) => "Sub",
      Token::Op(Operator::Mul) => "Mul",
      Token::Op(Operator::Equ) => "Equ",
      Token::Op(Operator::Leq) => "Leq",
      Token::Op(Operator::Not) => "Not",
      Token::Op(Operator::And) => "And",
      Token::Op(Operator::Semicolon) => "Semicolon",
      Token::Del(Delimiter::LParen) => "LParen",
      Token::Del(Delimiter::RParen) => "RParen",
      Token::Lit(Literal::True) => "True",
      Token::Lit(Literal::False) => "False",
      Token::Lit(Literal::Int(_)) => "Int",
      Token::Idf(Identifier::Variable(_)) => "Variable",
      Token::End => "End",
    }
  }

  fn kinds(tokens: &[Token]) -> Vec<&'static str> {
    tokens.iter().map(token_kind).collect()
  }

  // -------------------------------------------------------------------------
  // lex_int
  // -------------------------------------------------------------------------

  #[test]
  fn lex_int_single_digit() {
    let chars: Vec<char> = "7".chars().collect();
    let (val, skip) = lex_int(&chars);
    assert_eq!(val, 7);
    assert_eq!(skip, 1);
  }

  #[test]
  fn lex_int_multi_digit() {
    let chars: Vec<char> = "42".chars().collect();
    let (val, skip) = lex_int(&chars);
    assert_eq!(val, 42);
    assert_eq!(skip, 2);
  }

  #[test]
  fn lex_int_stops_at_non_digit() {
    let chars: Vec<char> = "123abc".chars().collect();
    let (val, skip) = lex_int(&chars);
    assert_eq!(val, 123);
    assert_eq!(skip, 3);
  }

  #[test]
  fn lex_int_large_number() {
    let chars: Vec<char> = "9999999".chars().collect();
    let (val, skip) = lex_int(&chars);
    assert_eq!(val, 9999999);
    assert_eq!(skip, 7);
  }

  #[test]
  fn lex_int_zero() {
    let chars: Vec<char> = "0".chars().collect();
    let (val, skip) = lex_int(&chars);
    assert_eq!(val, 0);
    assert_eq!(skip, 1);
  }

  // -------------------------------------------------------------------------
  // Operators
  // -------------------------------------------------------------------------

  #[test]
  fn lex_assign() {
    let tokens = lex(":=").unwrap();
    assert_eq!(kinds(&tokens), vec!["Assign", "End"]);
  }

  #[test]
  fn lex_leq() {
    let tokens = lex("<=").unwrap();
    assert_eq!(kinds(&tokens), vec!["Leq", "End"]);
  }

  #[test]
  fn lex_equ() {
    let tokens = lex("=").unwrap();
    assert_eq!(kinds(&tokens), vec!["Equ", "End"]);
  }

  #[test]
  fn lex_add() {
    let tokens = lex("+").unwrap();
    assert_eq!(kinds(&tokens), vec!["Add", "End"]);
  }

  #[test]
  fn lex_sub() {
    let tokens = lex("-").unwrap();
    assert_eq!(kinds(&tokens), vec!["Sub", "End"]);
  }

  #[test]
  fn lex_mul() {
    let tokens = lex("*").unwrap();
    assert_eq!(kinds(&tokens), vec!["Mul", "End"]);
  }

  #[test]
  fn lex_not() {
    let tokens = lex("!").unwrap();
    assert_eq!(kinds(&tokens), vec!["Not", "End"]);
  }

  #[test]
  fn lex_and() {
    let tokens = lex("&").unwrap();
    assert_eq!(kinds(&tokens), vec!["And", "End"]);
  }

  #[test]
  fn lex_semicolon() {
    let tokens = lex(";").unwrap();
    assert_eq!(kinds(&tokens), vec!["Semicolon", "End"]);
  }

  // -------------------------------------------------------------------------
  // Delimiters
  // -------------------------------------------------------------------------

  #[test]
  fn lex_parens() {
    let tokens = lex("()").unwrap();
    assert_eq!(kinds(&tokens), vec!["LParen", "RParen", "End"]);
  }

  // -------------------------------------------------------------------------
  // Keywords
  // -------------------------------------------------------------------------

  #[test]
  fn lex_keyword_if() {
    let tokens = lex("if").unwrap();
    assert_eq!(kinds(&tokens), vec!["If", "End"]);
  }

  #[test]
  fn lex_keyword_then() {
    let tokens = lex("then").unwrap();
    assert_eq!(kinds(&tokens), vec!["Then", "End"]);
  }

  #[test]
  fn lex_keyword_else() {
    let tokens = lex("else").unwrap();
    assert_eq!(kinds(&tokens), vec!["Else", "End"]);
  }

  #[test]
  fn lex_keyword_while() {
    let tokens = lex("while").unwrap();
    assert_eq!(kinds(&tokens), vec!["While", "End"]);
  }

  #[test]
  fn lex_keyword_do() {
    let tokens = lex("do").unwrap();
    assert_eq!(kinds(&tokens), vec!["Do", "End"]);
  }

  #[test]
  fn lex_keyword_skip() {
    let tokens = lex("skip").unwrap();
    assert_eq!(kinds(&tokens), vec!["Skip", "End"]);
  }

  #[test]
  fn lex_keyword_end() {
    let tokens = lex("end").unwrap();
    assert_eq!(kinds(&tokens), vec!["End", "End"]);
  }

  // -------------------------------------------------------------------------
  // Literals
  // -------------------------------------------------------------------------

  #[test]
  fn lex_literal_true() {
    let tokens = lex("true").unwrap();
    assert_eq!(kinds(&tokens), vec!["True", "End"]);
  }

  #[test]
  fn lex_literal_false() {
    let tokens = lex("false").unwrap();
    assert_eq!(kinds(&tokens), vec!["False", "End"]);
  }

  #[test]
  fn lex_integer_literal() {
    let tokens = lex("42").unwrap();
    assert_eq!(kinds(&tokens), vec!["Int", "End"]);
    if let Token::Lit(Literal::Int(n)) = &tokens[0] {
      assert_eq!(*n, 42);
    } else {
      panic!("expected Int literal");
    }
  }

  // -------------------------------------------------------------------------
  // Whitespace
  // -------------------------------------------------------------------------

  #[test]
  fn lex_skips_spaces() {
    let tokens = lex("+ +").unwrap();
    assert_eq!(kinds(&tokens), vec!["Add", "Add", "End"]);
  }

  #[test]
  fn lex_skips_newlines_and_tabs() {
    let tokens = lex("+\n\t\r+").unwrap();
    assert_eq!(kinds(&tokens), vec!["Add", "Add", "End"]);
  }

  // -------------------------------------------------------------------------
  // Sequences
  // -------------------------------------------------------------------------

  #[test]
  fn lex_assignment_statement() {
    // x := 5
    let tokens = lex("x := 5").unwrap();
    assert_eq!(kinds(&tokens), vec!["Variable", "Assign", "Int", "End"]);
  }

  #[test]
  fn lex_sequence_with_semicolon() {
    // x := 1 ; y := 2
    let tokens = lex("x := 1 ; y := 2").unwrap();
    assert_eq!(
      kinds(&tokens),
      vec![
        "Variable",
        "Assign",
        "Int",
        "Semicolon",
        "Variable",
        "Assign",
        "Int",
        "End"
      ]
    );
  }

  #[test]
  fn lex_while_loop() {
    let tokens = lex("while x do skip end").unwrap();
    assert_eq!(
      kinds(&tokens),
      vec!["While", "Variable", "Do", "Skip", "End", "End"]
    );
  }

  #[test]
  fn lex_if_statement() {
    let tokens = lex("if true then skip else skip end").unwrap();
    assert_eq!(
      kinds(&tokens),
      vec!["If", "True", "Then", "Skip", "Else", "Skip", "End", "End"]
    );
  }

  #[test]
  fn lex_leq_vs_assign_no_confusion() {
    // '<=' and ':=' must not be confused with '<' or ':'
    let tokens = lex("<= :=").unwrap();
    assert_eq!(kinds(&tokens), vec!["Leq", "Assign", "End"]);
  }

  // -------------------------------------------------------------------------
  // EOF / empty input
  // -------------------------------------------------------------------------

  #[test]
  fn lex_empty_string() {
    let tokens = lex("").unwrap();
    assert_eq!(kinds(&tokens), vec!["End"]);
  }

  #[test]
  fn lex_only_whitespace() {
    let tokens = lex("   \n\t  ").unwrap();
    assert_eq!(kinds(&tokens), vec!["End"]);
  }

  // -------------------------------------------------------------------------
  // Error cases
  // -------------------------------------------------------------------------

  #[test]
  fn lex_unidentified_token_returns_err() {
    let result = lex("@");
    assert!(result.is_err());
    if let Err(UserDefinedError::Lex(LexError::UnidentifiedToken(c))) = result {
      assert_eq!(c, '@');
    } else {
      panic!("expected UnidentifiedToken('@')");
    }
  }

  #[test]
  fn lex_unidentified_token_in_middle_returns_err() {
    // Valid tokens before the bad one — should still error
    let result = lex("x := @");
    assert!(result.is_err());
  }

  #[test]
  fn lex_bare_colon_returns_err() {
    // ':' without '=' is not a valid token
    let result = lex(":");
    assert!(result.is_err());
  }
}
