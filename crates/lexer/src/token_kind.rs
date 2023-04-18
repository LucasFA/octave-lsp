use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt;
use strum_macros::EnumIter;

/// The kind of a token produced by the lexer.
/// It is called this for consistency with Rowan, the parsing library.
#[derive(
    Logos,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumIter,
    ToPrimitive,
    FromPrimitive,
)]
#[repr(u16)]
// #[logos(subpattern close_block_comment = r##"(#|%)\}"##)]
// #[logos(subpattern open_block_comment = r##"(#|%)\{"##)]
pub enum TokenKind {
    // This would be ideal for block comments, but logos doesn't support non-greedy regexes
    // #[regex(r##"(#|%)\{\s*\n(.|\n)*?(#|%)\}"##)]
    // Maybe we can use callbacks?
    // The current solution matches a bit more than it should, but it actually allows us to parse better
    // It also matches block comments that end in "#} foobar" (which is a line which should not end it, technically, but is most likely user error)
    // #[regex(r##"(.*[#%]\{.*\n)((?:[^#%]|[#%][^}])*)([#%]\})"##)] // well this breaks everything
    #[regex("[#%].*")]
    Comment,

    #[token(";")]
    Semicolon,

    #[token("\n")]
    Newline,

    #[regex("[ \r\t]+")]
    Whitespace,

    // Refer to https://docs.octave.org/v7.3.0/Keywords.html
    // for a list of reserved keywords
    #[token("function")]
    FnKw,
    #[token("endfunction")]
    EndFnKw,
    #[token("if")]
    IfKw,
    #[token("elseif")]
    ElseIfKw,
    #[token("else")]
    ElseKw,
    #[token("endif")]
    EndIfKw,
    #[token("switch")]
    SwitchKw,
    #[token("case")]
    CaseKw,
    #[token("otherwise")]
    OtherwiseKw,
    #[token("endswitch")]
    EndSwitchKw,
    #[token("while")]
    WhileKw,
    #[token("endwhile")]
    EndWhileKw,
    #[token("do")]
    DoKw,
    #[token("until")]
    UntilKw,
    #[token("for")]
    ForKw,
    #[token("endfor")]
    EndForKw,
    #[token("break")]
    BreakKw,
    #[token("continue")]
    ContinueKw,
    #[token("unwind_protect")]
    UnwindProtectKw,
    #[token("unwind_protect_cleanup")]
    UnwindProtectCleanupKw,
    #[token("end_unwind_protect")]
    EndUnwindProtectKw,
    #[token("try")]
    TryKw,
    #[token("catch")]
    CatchKw,
    #[token("end_try_catch")]
    EndTryKw,
    #[token("end")]
    EndKw,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token(".*")]
    ElmtMult,
    #[token("/")]
    Slash,
    #[token("./")]
    ElmtDiv,
    #[token(r"\")]
    LeftDiv,
    #[token(r".\")]
    ElmtLeftDiv,
    #[token("^")]
    Caret,
    #[token(".^")]
    ElmtPow,
    #[token("'")]
    Transpose,
    #[token(".'")]
    ElmtTranspose,
    #[token("!")]
    Not,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("==")]
    EqualsEquals,
    #[token("!=")]
    NotEquals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessThanEquals,
    #[token(">=")]
    GreaterThanEquals,
    #[token("=")]
    Equals,

    #[token(":")]
    Colon,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // The name of a variable must be a sequence of letters, digits and underscores, but it may not begin with a digit.
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    // A number is a sequence of digits, possibly containing a decimal point.
    #[regex(r"\d+")] // Integers
    #[regex(r"\d*\.\d*")] // Floats
    Number,

    #[error]
    Error,

    #[doc(hidden)]
    __LAST,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    pub fn is_keyword_statement(&self) -> bool {
        (TokenKind::FnKw..=TokenKind::EndKw).contains(self)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::FnKw => "'fn'",
            Self::Identifier => "identifier",
            Self::Number => "number",
            Self::Plus => "'+'",
            Self::Minus => "'-'",
            Self::Asterisk => "'*'",
            Self::Slash => "'/'",
            Self::Equals => "'='",
            Self::LParen => "'('",
            Self::RParen => "')'",
            Self::LBrace => "'{'",
            Self::RBrace => "'}'",
            Self::Comment => "comment",
            Self::Error => "an unrecognized token",
            Self::Semicolon => "';'",
            _ => todo!("Not yet implemented fmt::Display for TokenKind variant"),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use strum::IntoEnumIterator;

    use crate::{Lexer, TokenKind};

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    #[test]
    fn lex_spaces() {
        check("   ", TokenKind::Whitespace);
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
        // Names that begin and end with two underscores are understood to be reserved for internal use by Octave. You should not use them in code you write, except to access Octave's documented internal variables and built-in symbolic constants.
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
    fn lex_newlines() {
        check("\n", TokenKind::Newline);
    }

    #[test]
    fn lex_comment_hash() {
        check("# foo", TokenKind::Comment);
    }

    #[test]
    fn lex_comment_percent() {
        check("% foo", TokenKind::Comment);
    }

    // TEST KEYWORDS

    #[test]
    fn lex_fn_keyword() {
        check("function", TokenKind::FnKw);
    }

    #[test]
    fn lex_endfn_keyword() {
        check("endfunction", TokenKind::EndFnKw);
    }

    #[test]
    fn lex_kw_if() {
        check("if", TokenKind::IfKw)
    }

    #[test]
    fn lex_kw_elseif() {
        check("elseif", TokenKind::ElseIfKw)
    }

    #[test]
    fn lex_kw_else() {
        check("else", TokenKind::ElseKw)
    }

    #[test]
    fn lex_kw_endif() {
        check("endif", TokenKind::EndIfKw)
    }
    #[test]
    fn lex_kw_switchkw() {
        check("switch", TokenKind::SwitchKw)
    }
    #[test]
    fn lex_kw_casekw() {
        check("case", TokenKind::CaseKw)
    }
    #[test]
    fn lex_kw_otherwisekw() {
        check("otherwise", TokenKind::OtherwiseKw)
    }
    #[test]
    fn lex_kw_endswitch() {
        check("endswitch", TokenKind::EndSwitchKw)
    }
    #[test]
    fn lex_kw_whilekw() {
        check("while", TokenKind::WhileKw)
    }
    #[test]
    fn lex_kw_endwhilekw() {
        check("endwhile", TokenKind::EndWhileKw)
    }
    #[test]
    fn lex_kw_dokw() {
        check("do", TokenKind::DoKw)
    }
    #[test]
    fn lex_kw_untilkw() {
        check("until", TokenKind::UntilKw)
    }
    #[test]
    fn lex_kw_forkw() {
        check("for", TokenKind::ForKw)
    }
    #[test]
    fn lex_kw_endforkw() {
        check("endfor", TokenKind::EndForKw)
    }
    #[test]
    fn lex_kw_breakkw() {
        check("break", TokenKind::BreakKw)
    }

    #[test]
    fn lex_kw_continuekw() {
        check("continue", TokenKind::ContinueKw)
    }

    #[test]
    fn lex_kw_unwindprotectkw() {
        check("unwind_protect", TokenKind::UnwindProtectKw)
    }

    #[test]
    fn lex_kw_unwindprotectcleanupkw() {
        check("unwind_protect_cleanup", TokenKind::UnwindProtectCleanupKw)
    }

    #[test]
    fn lex_kw_endunwindprotectkw() {
        check("end_unwind_protect", TokenKind::EndUnwindProtectKw)
    }

    #[test]
    fn lex_kw_trykw() {
        check("try", TokenKind::TryKw)
    }

    #[test]
    fn lex_kw_catchkw() {
        check("catch", TokenKind::CatchKw)
    }

    #[test]
    fn lex_kw_endtrykw() {
        check("end_try_catch", TokenKind::EndTryKw)
    }

    #[test]
    fn lex_kw_endkw() {
        check("end", TokenKind::EndKw)
    }

    // Test with full coverage. An error here would be hard to debug
    fn to_kw_or_not_to_kw() -> ([TokenKind; 25], [TokenKind; 37]) {
        use crate::TokenKind::*;
        let keywords = [
            FnKw,
            EndFnKw,
            IfKw,
            ElseIfKw,
            ElseKw,
            EndIfKw,
            SwitchKw,
            CaseKw,
            OtherwiseKw,
            EndSwitchKw,
            WhileKw,
            EndWhileKw,
            DoKw,
            UntilKw,
            ForKw,
            EndForKw,
            BreakKw,
            ContinueKw,
            UnwindProtectKw,
            UnwindProtectCleanupKw,
            EndUnwindProtectKw,
            TryKw,
            CatchKw,
            EndTryKw,
            EndKw,
        ];

        let non_kw = [
            Whitespace,
            Newline,
            Semicolon,
            Identifier,
            Number,
            Plus,
            Minus,
            Asterisk,
            ElmtMult,
            Slash,
            ElmtDiv,
            LeftDiv,
            ElmtLeftDiv,
            Caret,
            ElmtPow,
            Transpose,
            ElmtTranspose,
            Not,
            And,
            Or,
            EqualsEquals,
            NotEquals,
            LessThan,
            GreaterThan,
            LessThanEquals,
            GreaterThanEquals,
            Equals,
            Colon,
            LParen,
            RParen,
            LBracket,
            RBracket,
            LBrace,
            RBrace,
            Comment,
            Error,
            __LAST,
        ];

        let u: HashSet<TokenKind> = keywords
            .to_owned()
            .into_iter()
            .chain(non_kw.to_owned().into_iter())
            .collect();
        let v: HashSet<TokenKind> = TokenKind::iter().collect();

        let dif: Vec<_> = u.symmetric_difference(&v).collect();
        assert_eq!(dif.len(), 0);

        (keywords, non_kw)
    }

    fn check_trivia(input: TokenKind, expected: bool) {
        assert_eq!(TokenKind::is_trivia(input), expected)
    }

    fn check_is_keyword(input: TokenKind, expected: bool) {
        assert_eq!(TokenKind::is_keyword_statement(&input), expected)
    }

    #[test]
    fn full_coverage() {
        use strum::IntoEnumIterator;
        let (kws, non_kws) = to_kw_or_not_to_kw();
        assert_eq!(kws.len() + non_kws.len(), TokenKind::iter().count())
    }

    #[test]
    fn check_trivias() {
        use strum::IntoEnumIterator;
        for val in TokenKind::iter() {
            check_trivia(
                val,
                [
                    TokenKind::Whitespace,
                    TokenKind::Newline,
                    TokenKind::Comment,
                ]
                .contains(&val),
            );
        }
    }

    #[test]
    fn check_yes_keywords() {
        let (keywords, _) = to_kw_or_not_to_kw();
        for val in keywords {
            check_is_keyword(val, true);
        }
    }

    #[test]
    fn check_no_keyword() {
        let (_, non_kws) = to_kw_or_not_to_kw();
        for val in non_kws {
            check_is_keyword(val, false);
        }
    }
}
