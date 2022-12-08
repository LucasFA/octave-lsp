//! This module contains the lexer for Octave.
//!
//! The lexer is implemented using the [logos](https://crates.io/crates/logos) crate.
//! In general terms, the lexer takes a string as input and produces a stream of tokens as output.
//! Those tokens are represented by the Token struct, which holds the kind of the token and the text that it holds.

mod token_kind;
pub use token_kind::TokenKind;

use logos::Logos;

/// Logos wrapper.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some(Token { kind, text: input }));
    }

    #[test]
    fn lex_spaces() {
        check("   ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("function", TokenKind::FnKw);
    }

    #[test]
    fn lex_endfn_keyword() {
        check("endfunction", TokenKind::EndFnKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", TokenKind::Identifier);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", TokenKind::Identifier);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", TokenKind::Identifier);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", TokenKind::Identifier);
    }

    #[test]
    fn lex_underscore_identifier() {
        // According to the GNU documentation,
        // Names that begin and end with two underscores are understood to be reserved for internal use by Octave. You should not use them in code you write, except to access Octave’s documented internal variables and built-in symbolic constants.
        check("__x__", TokenKind::Identifier);
    }

    #[test]
    fn lex_number() {
        check("123456", TokenKind::Number);
    }

    #[test]
    fn lex_float() {
        check("123.456", TokenKind::Number);
    }

    #[test]
    fn lex_float2() {
        check(".456", TokenKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", TokenKind::Asterisk);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", TokenKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check("{", TokenKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", TokenKind::RBrace);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", TokenKind::RParen);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_comment_hash() {
        check("# foo", TokenKind::Comment);
    }

    #[test]
    fn lex_comment_percent() {
        check("% foo", TokenKind::Comment);
    }
}
