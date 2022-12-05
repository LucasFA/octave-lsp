// Generated from grammar.ron
use crate::tree::{SyntaxKind, SyntaxInfo};

pub const IDENTIFIER: SyntaxKind = SyntaxKind(0);
pub const WHITE_SPACE: SyntaxKind = SyntaxKind(1);

static IDENTIFIER_INFO: SyntaxInfo = SyntaxInfo {
   name: "IDENTIFIER",
};
static WHITE_SPACE_INFO: SyntaxInfo = SyntaxInfo {
   name: "WHITE_SPACE",
};

pub(crate) fn syntax_info(kind: SyntaxKind) -> &'static SyntaxInfo {
    match kind {
        IDENTIFIER => &IDENTIFIER_INFO,
        WHITE_SPACE => &WHITE_SPACE_INFO,
        _ => unreachable!()
    }
}
