use super::Parser;

pub(super) fn expr(p: &mut Parser) {
    if let Some(_) = p.peek() {
        p.bump();
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
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
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
