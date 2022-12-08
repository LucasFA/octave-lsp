//! Defines the equivalence from Lexical tokens to Syntactical tokens.
//! 

use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,
    FnKw,
    EndFnKw,
    LetKw,
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
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::FnKw => Self::FnKw,
            TokenKind::EndFnKw => Self::EndFnKw,
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
