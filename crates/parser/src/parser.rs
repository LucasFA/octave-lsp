pub(crate) mod marker;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use marker::Marker;
use syntax::SyntaxKind;

const RECOVERY_SET: [SyntaxKind; 1] = [SyntaxKind::Semicolon];

pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    pub(crate) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub(crate) fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }

    pub(crate) fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    pub(crate) fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(crate) fn error(&mut self) {
        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            let m = self.start();
            self.bump();
            m.complete(self, SyntaxKind::Error);
        }
    }

    fn at_set(&mut self, set: &[SyntaxKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub(crate) fn at_end(&mut self) -> bool {
        println!("Are we at the end?");
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_comment() {
        check(
            "# hello!",
            expect![[r##"
Root@0..8
  Comment@0..8 "# hello!""##]],
        );
    }
    // This doesn't yet pass
    //     #[test]
    //     fn parse_block_comment() {
    //         check(
    //             "#{
    //     hello!
    //     %}",
    //         expect![[r##"
    // Root@0..23
    //   BlockComment@0..23 "#{
    //     hello!
    //     %}"##]]);
    //     }
}
