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
