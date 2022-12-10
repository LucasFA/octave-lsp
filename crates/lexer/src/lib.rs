//! This module contains the lexer for Octave.
//!
//! The lexer is implemented using the [logos](https://crates.io/crates/logos) crate.
//! In general terms, the lexer takes a string as input and produces a stream of tokens as output.
//! Those tokens are represented by the Token struct, which holds the kind of the token and the text that it holds.

mod token_kind;
pub use token_kind::TokenKind;

use logos::Logos;

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

/// The output of the lexer. It contains the kind of the token and the text associated.
#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}
