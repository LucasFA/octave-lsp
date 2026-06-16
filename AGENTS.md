# octave-lsp — agent instructions

## Workspace

Rust workspace (`resolver = "3"`), members in `crates/*`. Edition **2024**.

## Crate architecture (dependency order)

```
lexer → syntax → parser → ast → hir → octave-lsp (binary)
```

- **lexer** — logos-based tokenizer, produces `Token` stream. Line comments `#` / `%`.
- **syntax** — rowan `SyntaxKind` + `SyntaxConstruct` enum, `OctaveLanguage` type.
- **parser** — Pratt parser with event/sink architecture (see: Rust Analyzer style). `parser::parse(&str) -> Parse`. Trivia reattached by `Sink`. `drop_bomb` ensures markers complete.
- **ast** — typed AST wrappers (`TypedSyntaxNode` trait) + `validation` module.
- **hir** — arena-based lowering (`la_arena`, `smol_str`), `Database` for expression nodes.
- **octave-lsp** — binary crate. Currently a **REPL** (stdin → parse → debug_tree), not yet an LSP server.

## Key commands

```sh
cargo build                    # all crates
cargo test                     # all tests
cargo test -p <crate>          # single crate
cargo clippy -- -D warnings    # lint (CI gates on this)
cargo fmt --all -- --check     # formatting check
cargo deny check               # advisory/license/bans
cargo depgraph --all-deps | dot -Tsvg > docs/graph.svg  # dep graph
```

## Testing

Snapshot tests via `expect_test` (`expect!` macro / `expect![[r#"..."#]]`).  
Inline in `#[cfg(test)] mod tests` beside the code. No external test framework.

```rust
// pattern used in parser, ast, hir crates:
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}
```

## Fuzz

Uses `cargo-fuzz` + `libfuzzer-sys`. Target: `parser → ast::validate → hir::lower`.

```sh
cargo fuzz run main -- -max_len=64
```

## CI pipeline (`.github/workflows/rust.yml`)

1. `cargo fmt --all -- --check`
2. `cargo clippy -- -D warnings`
3. `cargo build --verbose`
4. `cargo test --verbose`

## VS Code extension

`editors/code/` — TypeScript, not yet published.  
Uses `vscode-languageclient`. Entry: `./out/extension`.

## Lint attributes

All crates use `#![warn(clippy::pedantic)]`.  
`octave-lsp` also uses `#![warn(missing_docs)]`.

## Current state

Work in progress. Not all Octave syntax is parsed. No LSP protocol implemented yet
(the binary is a REPL that reads lines, parses, validates, and prints the debug tree).

## Test-first workflow

```sh
cargo test -p <crate> -- <test_name>        # single test
UPDATE_EXPECT=1 cargo test -p <crate> -- ... # update snapshots
```

## Pipeline overview (conceptual location of each concept)

| Concept | Crate | Notes |
|---|---|---|
| `+` token exists | `lexer` | `TokenKind::Plus` |
| `+` in expression tree | `parser` | Emitted as `InfixExpr` with `Plus` child token |
| `+` in CST node type | `syntax` | `SyntaxConstruct::InfixExpr` |
| `a + b` as typed node | `ast` | `Expr::BinaryExpr` |
| `a + b` as owned data | `hir` | `Expr::Binary { op: BinaryOp::Add, ... }` |

## `InfixExpr` policy

All binary operators share `SyntaxConstruct::InfixExpr`. The `ast` layer
inspects the operator token kind to classify (`Expr::BinaryExpr` vs future
`Expr::CompareExpr` vs `Expr::LogicExpr`). This avoids exploding the
`SyntaxConstruct` enum without losing semantic precision.

---

## Build plan: bottom-up Octave syntax coverage

### Phase 1: Lexer (`crates/lexer/src/token_kind.rs`)

**1a. Fix float regex + add scientific notation.** Current `\d+` + `\d*\.\d*`
fragments `123.456` into two tokens and matches bare `.`. Replace with one
regex that handles integers, floats, and scientific notation atomically:
`\d+\.?\d*([eE][+-]?\d+)?`

**1b. Add `Comma`** — `#[token(",")]`

**1c. Add `StringLiteral`** — single-quoted strings `'hello'`. Keep `'` as
`Transpose`; parser will disambiguate string-start vs. transpose-postfix by
context (string at expression start; transpose after an expression).

**1d. Add `Tilde` + `TildeEquals`** — `#[token("~")]` and `#[token("~=")]`

**1e. Add `At`** — `#[token("@")]` — function handles, anonymous functions

**1f. Add compound assignment** — `+=`, `-=`, `*=`, `/=`, `.*=`, `./=`, `.^=`

### Phase 2: Syntax constructs (`crates/syntax/src/lib.rs`)

Add `SyntaxConstruct` variants (no parser changes yet):

`MatrixExpr`, `CallExpr`, `PostfixExpr`, `RangeExpr`, `Block`,
`FnDef`, `IfStmt`, `ForLoop`, `WhileLoop`, `BreakStmt`, `ContinueStmt`

### Phase 3: Parser grammar (`crates/parser/src/grammar/`)

**3a. Extend binding power table** (`expr.rs`) — add all unused operators to
the Pratt loop. Power operators right-associative (left bp < right bp).

**3b. Matrix/cell literals** — `[ ]` with comma/semicolon-separated rows.

**3c. Function calls** — after LHS check for `(` → comma-separated args.

**3d. Control flow** — `if`/`elseif`/`else`/`endif`, `for`/`endfor`,
`while`/`endwhile`, `switch`/`case`/`endswitch`, `break`, `continue`.

**3e. Function definitions** — `function [out] = name(in) ... endfunction`.

### Phase 4: AST wrappers (`crates/ast/src/lib.rs`)

`impl_typed_syntax_node!` per new construct. Accessor methods. Extend
`Expr::cast()` to classify `InfixExpr` by operator token kind.

### Phase 5: HIR lowering (`crates/hir/src/`)

Extend `Expr`, `Stmt`, `BinaryOp`, `UnaryOp` enums. Add lowering functions.
Generalize `Literal` beyond `u64`. Add arenas as needed to `Database`.
