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
#[repr(u16)]
pub enum SyntaxKind {
    Comment,
    Semicolon,
    Newline,
    Whitespace,

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

    // Operators
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

    //Puntuaction
    Colon,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Identifier,
    Number,

    // Syntax-level nodes
    Error,
    Root,
    InfixExpr,
    ParenExpr,
    PrefixExpr,
    Literal,
    VariableRef,
    VariableDef,
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
            TokenKind::ElmtMult => Self::ElmtMult,
            TokenKind::Slash => Self::Slash,
            TokenKind::ElmtDiv => Self::ElmtDiv,
            TokenKind::LeftDiv => Self::LeftDiv,
            TokenKind::ElmtLeftDiv => Self::ElmtLeftDiv,
            TokenKind::Caret => Self::Caret,
            TokenKind::ElmtPow => Self::ElmtPow,
            TokenKind::Transpose => Self::Transpose,
            TokenKind::ElmtTranspose => Self::ElmtTranspose,
            TokenKind::Not => Self::Not,
            TokenKind::And => Self::And,
            TokenKind::Or => Self::Or,
            TokenKind::EqualsEquals => Self::EqualsEquals,
            TokenKind::NotEquals => Self::NotEquals,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::LessThanEquals => Self::LessThanEquals,
            TokenKind::GreaterThanEquals => Self::GreaterThanEquals,
            TokenKind::Equals => Self::Equals,
            TokenKind::Colon => Self::Colon,

            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBracket => Self::LBracket,
            TokenKind::RBracket => Self::RBracket,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::Comment => Self::Comment,

            TokenKind::Error => Self::Error,
            TokenKind::__LAST => unreachable!(r#"Tried to cast "__LAST": TokenKind to SyntaxKind"#),
        }
    }
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;

    use crate::SyntaxKind;
    use crate::SyntaxKind::*;

    // Test with full coverage. An error here would be hard to debug
    fn to_kw_or_not_to_kw_syntax() -> ([SyntaxKind; 8], Vec<SyntaxKind>) {
        let syntax_nodes = [
            Error,
            Root,
            InfixExpr,
            Literal,
            ParenExpr,
            PrefixExpr,
            VariableRef,
            VariableDef,
        ];

        let rest = SyntaxKind::iter()
            .filter(|x| !syntax_nodes.contains(x))
            .collect();

        (syntax_nodes, rest)
    }

    #[test]
    fn full_coverage() {
        use strum::IntoEnumIterator;
        let (syntax_nodes, non_syntax_nodes) = to_kw_or_not_to_kw_syntax();
        assert_eq!(
            syntax_nodes.len() + non_syntax_nodes.len(),
            SyntaxKind::iter().count()
        )
    }
}
