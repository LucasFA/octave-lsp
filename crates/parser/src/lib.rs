//! This module contains the parser for the Octave language.
#![warn(clippy::pedantic)]
mod event;
mod grammar;
mod parser;
mod sink;
mod source;

use crate::parser::{ParseError, Parser};
use lexer::Lexer;
use rowan::GreenNode;
use sink::Sink;
use source::Source;
use syntax::SyntaxNode;

/// Actually parses the input into a tree of `SyntaxNode`s.
#[must_use]
pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    #[must_use]
    pub fn debug_tree(&self) -> String {
        use std::fmt::Write as _; // import without risk of name clashing
        let mut s = String::new();

        let tree = format!("{:#?}", self.syntax());

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        s.push_str(&tree[0..tree.len() - 1]);
        for error in &self.errors {
            write!(s, "\n{error}").unwrap();
        }

        s
    }

    #[must_use]
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}
