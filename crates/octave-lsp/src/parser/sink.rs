use super::event::Event;
use crate::lexer::{Lexeme, SyntaxKind};
use crate::syntax::OctaveLanguage;
use rowan::{GreenNode, GreenNodeBuilder, Language};
use smol_str::SmolStr;
use std::mem;
pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
    events: Vec<Event>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            cursor: 0,
            events,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    if let Some(fp) = forward_parent {
                        if let Event::StartNode { kind, .. } = self.events[idx + fp] {
                            self.builder.start_node(OctaveLanguage::kind_to_raw(kind));
                        } else {
                            unreachable!()
                        }
                    }

                    self.builder.start_node(OctaveLanguage::kind_to_raw(kind));
                }
                Event::StartNodeAt { .. } => unreachable!(),
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
                Event::Placeholder => unreachable!(),
            }

            self.eat_trivia();
        }

        self.builder.finish()
    }
     fn eat_trivia(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if !lexeme.kind.is_trivia() {
                break;
            }

            self.token(lexeme.kind, lexeme.text.into());
        }
    }
    
    fn token(&mut self, kind: SyntaxKind, text: SmolStr) {
        self.builder.token(OctaveLanguage::kind_to_raw(kind), text.as_str());
        self.cursor += 1;
    }

}
