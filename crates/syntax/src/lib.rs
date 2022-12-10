//! Defines the equivalence from Lexical tokens to Syntactical tokens.

use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OctaveLanguage {}

impl rowan::Language for OctaveLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<OctaveLanguage>;
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive, EnumIter,
)]
pub enum SyntaxKind {
    Whitespace,

    Newline,
    Semicolon,

    // Statement Keywords
    FnKw,
    EndFnKw,
    IfKw,
    ElseIfKw,
    ElseKw,
    EndIfKw,
    SwitchKw,
    CaseKw,
    OtherwiseKw,
    EndSwitch,
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

    Identifier,
    Number,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comment,
    Error,
    Root,
    InfixExpr,
    Literal,
    ParenExpr,
    PrefixExpr,
    VariableRef,
    VariableDef,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    pub fn is_keyword_statement(&self) -> bool {
        (SyntaxKind::FnKw..=SyntaxKind::EndKw).contains(self)
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Newline => Self::Newline,
            TokenKind::Semicolon => Self::Semicolon,

            // Keywords
            TokenKind::FnKw => Self::FnKw,
            TokenKind::EndFnKw => Self::EndFnKw,
            TokenKind::IfKw => Self::IfKw,
            TokenKind::ElseIfKw => Self::ElseIfKw,
            TokenKind::ElseKw => Self::ElseKw,
            TokenKind::EndIfKw => Self::EndIfKw,
            TokenKind::SwitchKw => Self::SwitchKw,
            TokenKind::CaseKw => Self::CaseKw,
            TokenKind::OtherwiseKw => Self::OtherwiseKw,
            TokenKind::EndSwitchKw => Self::EndSwitch,
            TokenKind::WhileKw => Self::WhileKw,
            TokenKind::EndWhileKw => Self::EndWhileKw,
            TokenKind::DoKw => Self::DoKw,
            TokenKind::UntilKw => Self::UntilKw,
            TokenKind::ForKw => Self::ForKw,
            TokenKind::EndForKw => Self::EndForKw,
            TokenKind::BreakKw => Self::BreakKw,
            TokenKind::ContinueKw => Self::ContinueKw,
            TokenKind::UnwindProtectKw => Self::UnwindProtectKw,
            TokenKind::UnwindProtectCleanupKw => Self::UnwindProtectCleanupKw,
            TokenKind::EndUnwindProtectKw => Self::EndUnwindProtectKw,
            TokenKind::TryKw => Self::TryKw,
            TokenKind::CatchKw => Self::CatchKw,
            TokenKind::EndTryKw => Self::EndTryKw,
            TokenKind::EndKw => Self::EndKw,

            TokenKind::Identifier => Self::Identifier,
            TokenKind::Number => Self::Number,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Asterisk => Self::Asterisk,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBracket => Self::LBracket,
            TokenKind::RBracket => Self::RBracket,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::Comment => Self::Comment,
            TokenKind::Error => Self::Error,
            TokenKind::__LAST => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::SyntaxKind;
    use crate::SyntaxKind::*;

    // Test with full coverage. An error here would be hard to debug
    fn to_kw_or_not_to_kw() -> ([SyntaxKind; 25], [SyntaxKind; 25]) {
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
            EndSwitch,
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

        let non_kws = [
            Whitespace,
            Newline,
            Semicolon,
            Identifier,
            Number,
            Plus,
            Minus,
            Asterisk,
            Slash,
            Equals,
            LParen,
            RParen,
            LBracket,
            RBracket,
            LBrace,
            RBrace,
            Comment,
            Error,
            Root,
            InfixExpr,
            Literal,
            ParenExpr,
            PrefixExpr,
            VariableRef,
            VariableDef,
        ];

        (keywords, non_kws)
    }

    fn check_trivia(input: SyntaxKind, expected: bool) {
        assert_eq!(SyntaxKind::is_trivia(input), expected)
    }

    fn check_is_keyword(input: SyntaxKind, expected: bool) {
        assert_eq!(SyntaxKind::is_keyword_statement(&input), expected)
    }

    #[test]
    fn full_coverage() {
        use strum::IntoEnumIterator;
        let (kws, non_kws) = to_kw_or_not_to_kw();
        assert_eq!(kws.len() + non_kws.len(), SyntaxKind::iter().count())
    }

    #[test]
    fn check_trivias() {
        use strum::IntoEnumIterator;
        for val in SyntaxKind::iter() {
            check_trivia(val, [Whitespace, Comment].contains(&val));
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
