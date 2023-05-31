//! Defines the equivalence from Lexical tokens to Syntactical tokens.
#![warn(clippy::pedantic)]

use std::fmt::Debug;

pub use lexer::TokenKind;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use static_assertions::const_assert;
use strum::EnumCount;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OctaveLanguage {}

pub type SyntaxToken = rowan::SyntaxToken<OctaveLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<OctaveLanguage>;

impl rowan::Language for OctaveLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::try_from(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<OctaveLanguage>;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    LexToken(TokenKind),
    SyntaxConstruct(SyntaxConstruct),
}

impl std::fmt::Debug for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LexToken(token) => write!(f, "{:?}", token),
            Self::SyntaxConstruct(construct) => write!(f, "{:?}", construct),
        }
    }
}

impl From<TokenKind> for SyntaxKind {
    #[inline]
    fn from(value: TokenKind) -> Self {
        SyntaxKind::LexToken(value)
    }
}

impl From<SyntaxConstruct> for SyntaxKind {
    #[inline]
    fn from(value: SyntaxConstruct) -> Self {
        SyntaxKind::SyntaxConstruct(value)
    }
}

const_assert!(TokenKind::COUNT <= u8::MAX as usize);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
pub enum SyntaxConstruct {
    Error = TokenKind::COUNT as u8,
    Root,
    InfixExpr,
    ParenExpr,
    PrefixExpr,
    Literal,
    VariableRef,
    VariableDef,
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(value: SyntaxKind) -> Self {
        match value {
            SyntaxKind::LexToken(value) => value as u16,
            SyntaxKind::SyntaxConstruct(value) => value as u16,
        }
    }
}

impl TryFrom<u16> for SyntaxKind {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (value as usize) < TokenKind::COUNT {
            TokenKind::try_from(
                u8::try_from(value).expect("Tried to convert too big an integer into u8."),
            )
            .map(SyntaxKind::LexToken)
            .map_err(|_impossible| "Should not exist")
        } else {
            SyntaxConstruct::try_from(u8::try_from(value).expect("u16 value provided too big to convert to u8. Could not cast into u8 SyntaxConstruct"))
                .map(SyntaxKind::SyntaxConstruct)
                .map_err(|_e| "u8 values should correspond to SyntaxConstruct variant. Found error")
        }
    }
}
