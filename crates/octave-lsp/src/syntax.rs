use crate::lexer::SyntaxKind;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum OctaveLanguage {}

impl rowan::Language for OctaveLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        // use the __LAST field to make sure we don't miss any variants
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub(crate) type SyntaxNode = rowan::SyntaxNode<OctaveLanguage>;
