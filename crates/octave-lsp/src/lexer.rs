//! This module contains the lexer for Octave.
//! 
//! The lexer is implemented using the [logos](https://crates.io/crates/logos) crate.
//! In general terms, the lexer takes a string as input and produces a stream of tokens as output.
//! Those tokens are represented by the Lexeme struct, which holds the kind of the token and the text that it holds.

use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

/// The kind of a token produced by the lexer.
/// It is called this for consistency with Rowan, the parsing library.
#[derive(
    Logos, Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive,
)]
#[repr(u16)]
// #[logos(subpattern close_block_comment = r##"(#|%)\}"##)]
// #[logos(subpattern open_block_comment = r##"(#|%)\{"##)]
pub(crate) enum SyntaxKind {
    Root,

    Literal,
    BinaryExpr,
    ParenExpr,
    PrefixExpr,
    VariableRef,

    // This would be ideal for block comments, but logos doesn't support non-greedy regexes
    // #[regex(r##"(#|%)\{\s*\n(.|\n)*?(#|%)\}"##)]
    // Maybe we can use callbacks?
    #[regex("#.*")]
    #[regex("%.*")]
    Comment,

    #[regex("[ \n\r\t]+")]
    Whitespace,

    #[token("function")]
    FnKw,
    #[token("endfunction")]
    EndFnKw,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("[")]
    LBrat,
    #[token("]")]
    RBrat,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // The name of a variable must be a sequence of letters, digits and underscores, but it may not begin with a digit.
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    // A number is a sequence of digits, possibly containing a decimal point.
    #[regex(r"\d+")] // Integers
    #[regex(r"\d*\.\d+")] // Floats
    Number,

    #[error]
    Error,

    #[doc(hidden)]
    __LAST,
}

impl SyntaxKind {
    pub(crate) fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}
/// Logos wrapper.
pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

/// The output of the lexer. It contains the kind of the token and the text associated.
#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme<'a> {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: &'a str,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Lexeme { kind, text: input }));
    }

    #[test]
    fn lex_spaces() {
        check("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("function", SyntaxKind::FnKw);
    }

    #[test]
    fn lex_endfn_keyword() {
        check("endfunction", SyntaxKind::EndFnKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_underscore_identifier() {
        // According to the GNU documentation,
        // Names that begin and end with two underscores are understood to be reserved for internal use by Octave. You should not use them in code you write, except to access Octave’s documented internal variables and built-in symbolic constants.
        check("__x__", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::Number);
    }

    #[test]
    fn lex_float() {
        check("123.456", SyntaxKind::Number);
    }

    #[test]
    fn lex_float2() {
        check(".456", SyntaxKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", SyntaxKind::Asterisk);
    }

    #[test]
    fn lex_slash() {
        check("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check("{", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", SyntaxKind::RParen);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_comment_hash() {
        check("# foo", SyntaxKind::Comment);
    }

    #[test]
    fn lex_comment_percent() {
        check("% foo", SyntaxKind::Comment);
    }
}
