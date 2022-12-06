use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Logos, Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive,
)]
#[repr(u16)]
pub(crate) enum SyntaxKind {
    Root,

    #[regex(" +")]
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
    #[regex(r"\d*\.?\d*")]
    Number,

    #[error]
    Error,

    #[doc(hidden)]
    __LAST,
}

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

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let slice = self.inner.slice();

        Some((kind, slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Some((kind, input)));
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
}
