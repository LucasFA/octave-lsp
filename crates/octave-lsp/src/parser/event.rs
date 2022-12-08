use crate::lexer::SyntaxKind;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken {
        kind: SyntaxKind,
        text: SmolStr,
    },
    FinishNode,
    Placeholder,
}
