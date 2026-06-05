# while-compiler

A compiler and interpreter for the While language, written in Rust from scratch.

## Why this exists

When studying compiler construction or program analysis, a recurring problem is that there is no ready-made While language compiler that hands you the intermediate artifacts you actually need. If you want to implement reaching definitions, you first have to build a lexer, then a parser, then label the AST, then construct the CFG, just to get to the interesting part. This project eliminates that setup cost.

The compiler is a fully working pipeline from source text to CFG, with individual flags that let you stop at any stage and inspect or export the output. You can stop at the token stream, the AST, or the control flow graph, then take those artifacts and continue from there. The JSON outputs are also written to disk so external tools can consume them directly.

This is intended as a learning tool for courses and self-study in compiler construction, formal language theory, and program analysis.

---

## The While Language

While is a small imperative language commonly used in formal methods and program analysis courses. It has arithmetic expressions, boolean expressions, assignment, sequencing, conditionals, and while loops. There are no functions, no arrays, no heap, and no I/O, just integer variables and control flow. That simplicity makes it ideal for studying analyses like reaching definitions, live variables, and constant propagation without noise from a full language.

### Grammar

```
-- Statements
Stmt        ::= SimpleStmt (';' Stmt)?

SimpleStmt  ::= 'if' BExpr 'then' Stmt 'else' Stmt 'end'
              | 'while' BExpr 'do' Stmt 'end'
              | 'skip'
              | Ident ':=' AExpr

-- Arithmetic (low to high precedence)
AExpr       ::= Term (('+' | '-') Term)*      -- left associative

Term        ::= Factor ('*' Factor)*           -- left associative

Factor      ::= '-' Factor                     -- unary minus
              | '(' AExpr ')'
              | Int
              | Ident

-- Boolean (low to high precedence)
BExpr       ::= BComp ('&' BComp)*             -- left associative

BComp       ::= AExpr ('=' | '<=') AExpr       -- relational
              | '!' BFactor
              | BFactor

BFactor     ::= '(' BExpr ')'
              | 'true'
              | 'false'
```

### Syntax rules

**Blocks use `end`, not braces.** The `then` and `do` keywords open a block for `if` and `while` respectively, and `end` closes it. There is no `begin` keyword and no braces.

**Parentheses are for expressions only.** The `(` and `)` delimiters only group arithmetic or boolean sub-expressions. They cannot wrap statement blocks. This removes the ambiguity between expression grouping and block delimiters.

**Sequencing with `;`.** Multiple statements are chained with `;`. Sequencing is right-associative: `s1 ; s2 ; s3` parses as `s1 ; (s2 ; s3)`.

**Integer arithmetic.** The only numeric type is `i64`. Arithmetic supports `+`, `-`, `*`, and unary minus. There is no division.

**Boolean operators.** The relational operators are `=` (equality) and `<=` (less than or equal). Logical operators are `!` (not) and `&` (and). There is no `or`; it can be expressed as `!((!a) & (!b))`.

**Variables.** Variable names are sequences of letters, digits, and underscores starting with a letter. Variables hold `i64` values. Variables must be assigned before use; the semantic analysis pass enforces this.

**`skip`.** The no-op statement. Equivalent to an empty block. Useful as the else branch of a one-sided conditional.

### Example programs

Factorial:
```
n := 6;
result := 1;
while !(n <= 0) do
  result := result * n;
  n := n - 1
end
```

Conditional with skip:
```
x := 10;
if x <= 5 then
  y := 1
else
  skip
end
```

Nested while:
```
i := 0;
sum := 0;
while i <= 10 do
  j := 0;
  while j <= i do
    sum := sum + j;
    j := j + 1
  end;
  i := i + 1
end
```

Expression grouping:
```
x := (3 + 4) * 2;
y := -x + 1
```

### Token reference

| Token kind | Examples |
|---|---|
| Keywords | `if` `then` `else` `while` `do` `end` `skip` |
| Boolean literals | `true` `false` |
| Assignment | `:=` |
| Arithmetic operators | `+` `-` `*` |
| Relational operators | `=` `<=` |
| Boolean operators | `!` `&` |
| Sequencing | `;` |
| Grouping | `(` `)` |
| Integer literals | `0` `1` `42` `-7` |
| Identifiers | `x` `result` `myVar` |

---

## Setup

**Requirements:** Rust stable toolchain. Install from [https://rustup.rs](https://rustup.rs) if you do not have it.

```
git clone <repo-url>
cd while-compiler
cargo build --release
```

The compiled binary will be at `target/release/while-compiler`.

For development builds:
```
cargo build
cargo test
```

### Dependencies

| Crate | Purpose |
|---|---|
| `clap` | CLI argument parsing |
| `pretty` | AST pretty-printing |
| `colored` | Terminal colour output |
| `serde` + `serde_json` | JSON serialisation for output files |

The lexer and parser are hand-written and do not use any external parsing library.

---

## Usage

```
while-compiler [OPTIONS] <FILE>
```

The compiler reads a `.wl` (or any text) source file and runs the pipeline. By default it runs through semantic analysis and reports success or any errors. Use the flags below to stop at a specific stage or to execute the program.

### Flags

**`--tokens`**

Lex the source file and print the token stream to stdout, then exit. Also writes `outputs/tokens.json`.

```
while-compiler examples/basic.wl --tokens
```

Output:
```
Token Stream:
Number of tokens lexed: 23

KIND            | TEXT
-----------------------------------
Keyword         | if
...
```

**`--ast`**

Parse the token stream and pretty-print the AST to stdout, then exit. The tree is printed with 2-space indentation and syntax highlighting in the terminal. Also writes `outputs/ast.json` containing the full AST as a JSON tree.

```
while-compiler examples/basic.wl --ast
```

**`--cfg`**

Build the control flow graph from the labelled AST and print the nodes and edges to stdout, then exit. Each node is a labelled elementary statement. Also writes `outputs/cfg.json`.

```
while-compiler examples/basic.wl --cfg
```

Output shows the node labels with their statement text, then the edges:
```
Nodes (Labels and Statements):
  1: Assign x 1 + 2
  2: If 1 = 2
  ...

Edges (Control Flow):
  1 --> 2
  2 --> 3
  2 --> 4
  ...
```

**`--run`**

Execute the program using the big-step interpreter and print the final memory store. Also writes `outputs/store.json`.

```
while-compiler examples/basic.wl --run
```

Output:
```
Executing Program:

Final Memory Store:
  x = 3
  y = 11
  w = 14
```

### Stopping at a stage

The flags act as pipeline stops. If you pass `--tokens`, the compiler stops after lexing and never builds an AST. If you pass `--ast`, it stops after parsing. This means you can run the compiler multiple times with different flags to get each artifact independently.

---

## Output files

All JSON output files are written to an `outputs/` directory created in the current working directory.

| File | Produced by | Contents |
|---|---|---|
| `outputs/tokens.json` | `--tokens` | Array of token objects with kind and value |
| `outputs/ast.json` | `--ast` | The full AST as a nested JSON tree |
| `outputs/cfg.json` | `--cfg` | CFG nodes (labels), edges (from/to pairs), and a statement map |
| `outputs/store.json` | `--run` | Final variable store as a key/value map |

The JSON output is pretty-printed and intended for consumption by external tools. If you are implementing your own analysis and just need the CFG in a structured form, run the compiler with `--cfg` and read `outputs/cfg.json`.

---

## Project structure

```
src/
  main.rs          entry point and CLI driver
  lexer.rs         hand-written lexer, produces Vec<Token>
  ast.rs           AST node definitions and recursive descent parser
  error.rs         error type hierarchy (LexError, SyntaxError, SemanticError)
  analyzer.rs      semantic analysis pass (uninitialised variable detection)
  interpreter.rs   big-step interpreter (eval_aexpr, eval_bexpr, exec_stmt)
  labeler.rs       assigns unique integer labels to every elementary statement
  labelled_ast.rs  the labelled AST type (StatementL, mirrors Statement)
  cfg.rs           CFG construction from the labelled AST
  helpers.rs       output formatting, token printing, JSON writing

examples/
  basic.wl         a short example program
```

---

## What is implemented

The compiler currently covers lexing, parsing, semantic analysis, interpretation, AST labelling, and CFG construction.

**Lexer.** Hand-written character-by-character scanner. Handles all tokens in the grammar including multi-character tokens (`:=`, `<=`). Whitespace and newlines are ignored. Produces a flat `Vec<Token>`.

**Parser.** Hand-written recursive descent parser using a slice-and-consumed-count pattern. Each parse function takes a token slice and returns the parsed node along with the number of tokens consumed. There is no parser struct or cursor object. Operator precedence is encoded structurally: `parse_aexpr` delegates to `parse_term` which delegates to `parse_factor`, so `*` naturally binds tighter than `+`. All orphaned keywords produce named errors with distinct exit codes.

**AST.** Three node types: `AExpression`, `BExpression`, and `Statement`. All recursive nodes use `Box<T>` for heap allocation. `Display` implementations produce single-line output for error messages. The `pretty` crate is used for indented multi-line rendering.

**Semantic analysis.** Single-pass AST walk that tracks which variables have been assigned. Reports an error if a variable is used before any assignment. For `if/else`, the set of initialised variables after the branch is the intersection of the two branches (conservative: a variable is considered initialised only if both branches assign it).

**Interpreter.** Big-step (natural semantics) evaluator. `eval_aexpr` and `eval_bexpr` evaluate expressions against a `Store` (a `HashMap<String, i64>`). `exec_stmt` threads the store through statements. While loops execute until the condition is false. There is no step counter or fuel limit yet.

**Labelled AST.** A second AST type (`StatementL`) where every elementary statement carries a unique integer label. A `label_ast` function does a pre-order walk over a `Statement` tree and produces a `StatementL` tree with labels assigned in execution order. The label counter starts at 1.

**CFG.** The `build_cfg` function does a structural walk over a `StatementL` and builds a directed graph of `(from: label, to: label)` edges. It returns the entry label and the set of exit labels for each subtree. Sequencing connects the exit labels of the first statement to the entry label of the second. If/while nodes split and merge correctly. The graph is stored as a `Cfg` struct with a node list, edge list, and a map from label to statement tokens.

---

## Error codes

The compiler exits with a non-zero code on any error. The codes are listed below in case you are driving the compiler from a script.

| Code | Meaning |
|---|---|
| 20 | Unrecognised character in source |
| 30 | Block termination undefined |
| 31 | `then` without `if` |
| 32 | `else` without `if` |
| 33 | `do` without `while` |
| 34 | `end` without a block |
| 35 | Invalid assignment |
| 36 | Expected `then` after if condition |
| 37 | Expected `else` after then-branch |
| 38 | Expected `do` after while condition |
| 39 | Expected `end` to close block |
| 40 | Unconsumed tokens (stray token or missing `;`) |
| 50 | Uninitialised variable |

---

## Running the tests

```
cargo test
```

Tests are inline in each module. The lexer, parser, AST, and helpers modules all have unit tests. The parser tests cover every expression and statement form, including error paths for missing keywords and unconsumed tokens.

---

## License

See LICENSE.
