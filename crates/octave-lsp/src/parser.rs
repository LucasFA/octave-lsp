use crate::lexer::{SyntaxKind, Lexer};
use crate::syntax::{OctaveLanguage, SyntaxNode};
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>, // Could this be 'a?
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        if let Some(_) = self.peek() {
            self.bump();
        }

        self.finish_node();

        Parse {
            green_node: self.builder.finish(),
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(& mut self) {
        let (kind, text) = self.lexer.next().unwrap();
        self.builder.token(
            OctaveLanguage::kind_to_raw(kind),
            text
        )
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(OctaveLanguage::kind_to_raw(kind));
    }
    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse();

        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_identifier() {
        check(
            "hello",
            expect![[r#"
Root@0..5
  Identifier@0..5 "hello""#]],
        );
    }

     #[test]
    fn parse_binding_usage() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Identifier@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_whitespace_and_identifier() {
        check(
            "he llo ",
            expect![[r#"
Root@0..7  
  Identifier@0..2 "he"
  Whitespace@2..3 " "
  Identifier@3..6 "llo""#]],
            );
        }
}
