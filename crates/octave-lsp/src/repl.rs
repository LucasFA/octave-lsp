//! Octave REPL: reads lines, parses, validates, and prints the debug tree.

use parser::parse;
use std::io::{self, Write};

/// Runs the Octave REPL on stdin/stdout.
pub fn run() -> io::Result<()> {
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
            println!("{error}");
        }

        let root: ast::Root = ast::TypedSyntaxNode::cast(parse.syntax()).unwrap();

        dbg!(
            root.stmts()
                .filter_map(|stmt| {
                    if let ast::Stmt::VariableDef(var_def) = stmt {
                        Some(var_def.value())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        );

        dbg!(hir::lower(&root));

        input.clear();
    }
}
