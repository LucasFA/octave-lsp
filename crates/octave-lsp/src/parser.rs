use crate::lexer::SyntaxKind;
use crate::syntax::OctaveLanguage;
use logos::Logos;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, SyntaxKind>,
    builder: GreenNodeBuilder<'static>, // Could this be 'a?
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: SyntaxKind::lexer(input),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);
        self.finish_node();

        Parse {
            green_node: self.builder.finish(),
        }
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

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::SyntaxNode;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse();
        let syntax_node = SyntaxNode::new_root(parse.green_node);

        let actual_tree = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        expected_tree.assert_eq(&actual_tree[0..actual_tree.len() - 1]);
    }
    
    // This should not pass TODO
    #[test]
    fn parse_nothing() {
        check(" hello 2783", expect![[r#"Root@0..0"#]]);
    }

    // And this doesn't pass for probably the same reason
    #[test]
    fn parse_identifier() {
        check("hello", expect![[r#"Root@0..5"#]]);
    }
}