//! This module contains the lexer for Octave.
//!
//! The lexer is implemented using the [logos](https://crates.io/crates/logos) crate.
//! In general terms, the lexer takes a string as input and produces a stream of tokens as output.
//! Those tokens are represented by the Token struct, which holds the kind of the token and the text that it holds.

#![warn(clippy::pedantic)]

mod token_kind;
pub use token_kind::TokenKind;

use logos::Logos;
use std::ops::Range as StdRange;
use text_size::{TextRange, TextSize};

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    #[must_use]
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
    pub range: TextRange,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        if let Ok(kind) = kind {
            let text = self.inner.slice();

            let range = {
                let StdRange { start, end } = self.inner.span();
                let start = TextSize::try_from(start).unwrap();
                let end = TextSize::try_from(end).unwrap();

                TextRange::new(start, end)
            };

            Some(Self::Item {
                kind,
                text,
                range,
            })
        } else {
            None
        }
    }
}
