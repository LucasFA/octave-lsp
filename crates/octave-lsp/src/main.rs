//! # Octave Language Server
//! This crate provides a language server for the Octave programming language.
//! You can read more about the language server protocol [here](https://microsoft.github.io/language-server-protocol/).
//! The architecture of this language server is documented in the [ARCHITECTURE](../../../docs/architecture.md) file.
#![warn(missing_docs)]

use parser::parse;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = parse(&input);
        println!("{}", parse.debug_tree());

        let syntax = parse.syntax();
        for error in ast::validation::validate(&syntax) {
            println!("{}", error);
        }
        
        let root = ast::Root::cast(parse.syntax()).unwrap();

        dbg!(root
            .stmts()
            .filter_map(|stmt| if let ast::Stmt::VariableDef(var_def) = stmt {
                Some(var_def.value())
            } else {
                None
            })
            .collect::<Vec<_>>());

        dbg!(hir::lower(root));

        input.clear();
    }
}
