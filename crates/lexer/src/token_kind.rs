use logos::Logos;

/// The kind of a token produced by the lexer.
/// It is called this for consistency with Rowan, the parsing library.
#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
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
}
