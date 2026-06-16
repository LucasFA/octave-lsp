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

`editors/code/` — TypeScript LSP client. Not yet published. The Rust binary
(`octave-lsp`) is the server; the extension is the client that wires it into VS Code.

### Current state (incomplete / outdated)

| File | Issue |
|---|---|
| `package.json` | Deps pinned to old versions: `vscode-languageclient ^8.0.2` (2022), `@types/node ^7.10.14` (2017), `typescript ^4.9.3`. No `scripts.build`/`scripts.watch`/`scripts.test`. `contributes: {}` is empty — no language registration, no grammar, no config schema. |
| `src/Ctx.ts` | Hardcoded server path `<extPath>/server/target/debug/octave-lsp-server[.exe]`. Wrong on two counts: the real binary name is `octave-lsp`, and the actual build output is `target/debug/octave-lsp` (one `target/` deep from the repo root, not nested under `server/`). Debug args `["--debug", "--inspect=6009"]` are unverified against the current server. |
| `src/Config.ts` | Wrapper exists but is unused — no settings registered under the `octave-lsp` namespace. |
| `tslint.json` | TSLint is archived/deprecated; the `tslint:recommended` ruleset has not been updated since 2019. |
| `out/` | Not built; not in `.gitignore`. |
| `.vscodeignore` | Missing. Without it, `vsce package` would ship `node_modules/`, `src/`, `tsconfig.json`, `tslint.json`, and `*.map` files into the `.vsix`. |

### Work plan

**Phase 1 — make the existing code build and load.**
- Add build scripts to `package.json`: `build: tsc -p .`, `watch: tsc -watch -p .`, `typecheck: tsc --noEmit`.
- Add `editors/code/.gitignore` for `out/` and `node_modules/`.
- Add `editors/code/.vscodeignore` mirroring the rust-analyzer pattern (exclude everything, re-include the bundle, the prebuilt server dir, `package.json`, `LICENSE`, `icon.png`).
- Update `@types/node` to `^20` and `typescript` to `^5.x` to match the Node version VS Code ships.
- Drop `tslint.json`; add `eslint` + `@typescript-eslint` and an `eslint.config.mjs`.

**Phase 2 — register the language and the server.**
- In `package.json` `contributes`:
  - `languages`: `{ id: "octave", extensions: [".m", ".octave-config"], aliases: ["Octave", "octave"] }`
  - `language-configuration`: comment markers (`#`, `%`), brackets, indentation rules, word patterns.
  - `configuration`: settings schema for `octave-lsp.server.path`, `octave-lsp.trace.server`.
- Fix `src/Ctx.ts` to:
  - Resolve the server path from `octave-lsp.server.path` setting, defaulting to `<extPath>/server/octave-lsp[.exe]` (matches the bundled path used by rust-analyzer's `server/` dir).
  - Use the correct binary name and platform suffix.
  - Drop the stale `--debug --inspect=6009` flags (or wire them behind a real debug build).

**Phase 3 — ship the server binary with the extension.**
- Add a release task to the repo's CI (or a separate workflow) that builds the `octave-lsp` binary for `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin` (universal), and `x86_64-pc-windows-msvc`, then copies them into `editors/code/server/<target>/octave-lsp[.exe]`.
- The extension's `Ctx.ts` picks the right binary at runtime based on `process.platform` / `process.arch`.
- For local development, override via `octave-lsp.server.path` setting pointing at `target/debug/octave-lsp`.

**Phase 4 — publish.**
- `vsce package` produces `octave-lsp-X.Y.Z.vsix` for direct install.
- `vsce publish` (or web upload) ships the same bundle to the marketplace.
- Add a `package.json` `scripts.package: "vsce package"` and `scripts.publish: "vsce publish"`.
- Add a `repository` and `license` field if missing (already present).

**Phase 5 — CI integration.**
- Extend `.github/workflows/rust.yml` (or add a separate `vscode.yml`) to:
  - Install Node 20.
  - `npm ci` and `npm run typecheck` on every PR.
  - (Optional) Build the `octave-lsp` release binary for linux and stash as an artifact for `.vsix` smoke testing.

### Distribution model

Mirrors rust-analyzer: ship a prebuilt server binary per platform inside the `.vsix` under `editors/code/server/`. No download-on-activation, no user-side `cargo install`. Users get a single-install experience from the marketplace.

## Lint attributes

All crates use `#![warn(clippy::pedantic)]`.  
`octave-lsp` also uses `#![warn(missing_docs)]`.

## Current state

- **LSP server implemented** in `crates/octave-lsp/src/server.rs` (lsp-server + lsp-types).
  Default mode is the LSP server on stdio; pass `--repl` for the REPL.
  Capabilities: full text-document sync, parser + AST validation diagnostics published
  via `textDocument/publishDiagnostics`.
- **Parser/HIR/AST** cover Octave expressions, control flow, function definitions,
  try/catch, unwind_protect, strings, and postfix transpose.
- **174 tests pass**, clippy clean (`-D warnings`).
- **VS Code extension** in `editors/code/` is a stub — see the work plan below.

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

## Notable gotchas

- `ast::Stmt::VariableDef` still exists as a wrapper, but it's only constructed
  for `InfixExpr`-with-`=` nodes that have a `VariableRef` LHS. Assignments with
  a non-`VariableRef` LHS (e.g. `1 = 2`) return `None` from `VariableDef::name()`
  to avoid panics. The parser never emits a dedicated `SyntaxConstruct::VariableDef`.

---

## Build plan: bottom-up Octave syntax coverage

### ✅ Phase 1: Lexer (`crates/lexer/src/token_kind.rs`)

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

### ✅ Phase 2: Syntax constructs (`crates/syntax/src/lib.rs`)

Add `SyntaxConstruct` variants (no parser changes yet):

`MatrixExpr`, `CallExpr`, `PostfixExpr`, `RangeExpr`, `Block`,
`FnDef`, `IfStmt`, `ForLoop`, `WhileLoop`, `BreakStmt`, `ContinueStmt`

### ✅ Phase 3a: Extend binding power table (`crates/parser/src/grammar/expr.rs`)

Add all unused operators to the Pratt loop. Power operators right-associative
(left bp < right bp). Assignment `=` handled as lowest-precedence infix
operator — `VariableDef` construct is no longer emitted by parser.

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
