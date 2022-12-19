use crate::lexer::{SyntaxKind, Token};

/// A wrapper around a list of tokens that provides a cursor and some convenience methods.
pub(super) struct Source<'l, 'input> {
    tokens: &'l [Token<'input>],
    cursor: usize,
}

impl<'l, 'input> Source<'l, 'input> {
    pub(super) fn new(tokens: &'l [Token<'input>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(super) fn next_token(&mut self) -> Option<&'l Token<'input>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, SyntaxKind::is_trivia)
    }

    pub(super) fn peek_kind(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    fn peek_kind_raw(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.cursor).map(|Token { kind, .. }| *kind)
    }
}
