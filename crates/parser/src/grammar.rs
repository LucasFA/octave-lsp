mod expr;
mod stmt;

use crate::parser::Parser;
use crate::parser::marker::CompletedMarker;
use lexer::TokenKind;
use syntax::SyntaxConstruct;
use syntax::SyntaxKind;

pub(crate) fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    while !p.at_end() {
        stmt::stmt(p);
    }

    m.complete(p, SyntaxKind::SyntaxConstruct(SyntaxConstruct::Root))
}
