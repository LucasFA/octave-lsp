use crate::lexer::{Lexeme, SyntaxKind};

/// A wrapper around a list of lexemes that provides a cursor and some convenience methods.
pub(super) struct Source<'l, 'input> {
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
}


impl<'l, 'input> Source<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self { lexemes, cursor: 0 }
    }

    pub(super) fn next_lexeme(&mut self) -> Option<&'l Lexeme<'input>> {
        self.eat_trivia();

        let lexeme = self.lexemes.get(self.cursor)?;
        self.cursor += 1;

        Some(lexeme)
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
        self.lexemes
            .get(self.cursor)
            .map(|Lexeme { kind, .. }| *kind)
    }
}