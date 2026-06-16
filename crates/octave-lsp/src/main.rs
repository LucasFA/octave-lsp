//! # Octave Language Server
//!
//! This crate provides a language server for the Octave programming language.
//! You can read more about the language server protocol [here](https://microsoft.github.io/language-server-protocol/).
//! The architecture of this language server is documented in the [ARCHITECTURE](../../../docs/architecture.md) file.
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

mod repl;
mod server;

fn main() -> anyhow::Result<()> {
    let is_repl = std::env::args().any(|a| a == "--repl");

    if is_repl {
        repl::run()?;
    } else {
        server::run()?;
    }

    Ok(())
}
