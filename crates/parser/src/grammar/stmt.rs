use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(SyntaxKind::Identifier) {
        Some(handle_variable(p))
    } else {
        expr::expr(p)
    }
}

fn handle_variable(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Identifier));
    let lhs = p.start();
    p.bump();

    if p.at(SyntaxKind::Equals) {
        p.bump();
        expr::expr(p);
        return lhs.complete(p, SyntaxKind::VariableDef);
    }

    if p.at_end() {
        return lhs.complete(p, SyntaxKind::VariableRef);
    }

    let lhs = lhs.complete(p, SyntaxKind::VariableRef);
    let m = lhs.precede(p);
    // get what it is and then
    p.bump();
    expr::expr(p);
    let m = m.complete(p, SyntaxKind::InfixExpr);
    m
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_variable_definition() {
        check(
            "foo = bar",
            expect![[r#"
Root@0..9
  VariableDef@0..9
    Identifier@0..3 "foo"
    Whitespace@3..4 " "
    Equals@4..5 "="
    Whitespace@5..6 " "
    VariableRef@6..9
      Identifier@6..9 "bar""#]],
        );
    }

    #[test]
    fn parse_with_semicolon() {
        check(
            "a = 43;",
            expect![[r#"
            Root@0..7
              VariableDef@0..7
                Identifier@0..1 "a"
                Whitespace@1..2 " "
                Equals@2..3 "="
                Whitespace@3..4 " "
                Literal@4..6
                  Number@4..6 "43"
                Semicolon@6..7 ";""#]],
        )
    }

    #[test]
    fn parse_multiple_statements_with_semicolon() {
        check(
            "a = 43; 7",
            expect![[r#"
            Root@0..9
              VariableDef@0..8
                Identifier@0..1 "a"
                Whitespace@1..2 " "
                Equals@2..3 "="
                Whitespace@3..4 " "
                Literal@4..6
                  Number@4..6 "43"
                Semicolon@6..7 ";"
                Whitespace@7..8 " "
              Literal@8..9
                Number@8..9 "7""#]],
        )
    }

    #[test]
    fn recover_on_semicolon_broken_expression() {
        check(
            "a = 43 + ;",
            expect![[r#"
            Root@0..10
              VariableDef@0..10
                Identifier@0..1 "a"
                Whitespace@1..2 " "
                Equals@2..3 "="
                Whitespace@3..4 " "
                InfixExpr@4..10
                  Literal@4..7
                    Number@4..6 "43"
                    Whitespace@6..7 " "
                  Plus@7..8 "+"
                  Whitespace@8..9 " "
                  Semicolon@9..10 ";""#]],
        )
    }

    #[test]
    fn recover_on_semicolon_broken_expression_with_continuation() {
        check(
            "a = 43 + ;b = a",
            expect![[r#"
            Root@0..15
              VariableDef@0..10
                Identifier@0..1 "a"
                Whitespace@1..2 " "
                Equals@2..3 "="
                Whitespace@3..4 " "
                InfixExpr@4..10
                  Literal@4..7
                    Number@4..6 "43"
                    Whitespace@6..7 " "
                  Plus@7..8 "+"
                  Whitespace@8..9 " "
                  Semicolon@9..10 ";"
              VariableDef@10..15
                Identifier@10..11 "b"
                Whitespace@11..12 " "
                Equals@12..13 "="
                Whitespace@13..14 " "
                VariableRef@14..15
                  Identifier@14..15 "a""#]],
        )
    }

    #[test]
    fn recover_on_semicolon() {
        check(
            "a = ;\nb = a",
            expect![[r#"
Root@0..11
  VariableDef@0..6
    Identifier@0..1 "a"
    Whitespace@1..2 " "
    Equals@2..3 "="
    Whitespace@3..4 " "
    Semicolon@4..5 ";"
    Newline@5..6 "\n"
  VariableDef@6..11
    Identifier@6..7 "b"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    VariableRef@10..11
      Identifier@10..11 "a""#]],
        );
    }

    #[test]
    fn parse_expression_with_variable_reference() {
        check(
            "1+a",
            expect![[r#"
Root@0..3
  InfixExpr@0..3
    Literal@0..1
      Number@0..1 "1"
    Plus@1..2 "+"
    VariableRef@2..3
      Identifier@2..3 "a""#]],
        )
    }

    #[test]
    fn parse_expression_with_variable_reference2() {
        check(
            "a+1",
            expect![[r#"
  Root@0..3
    InfixExpr@0..3
      VariableRef@0..1
        Identifier@0..1 "a"
      Plus@1..2 "+"
      Literal@2..3
        Number@2..3 "1""#]],
        )
    }

    #[test]
    fn parse_multiple_statements() {
        check(
            "a = 1\na",
            expect![[r#"
Root@0..7
  VariableDef@0..6
    Identifier@0..1 "a"
    Whitespace@1..2 " "
    Equals@2..3 "="
    Whitespace@3..4 " "
    Literal@4..6
      Number@4..5 "1"
      Newline@5..6 "\n"
  VariableRef@6..7
    Identifier@6..7 "a""#]],
        );
    }

    #[test]
    fn parse_long_expression() {
        check(
            "a + b + 1",
            expect![[r#"
Root@0..9
  InfixExpr@0..9
    VariableRef@0..2
      Identifier@0..1 "a"
      Whitespace@1..2 " "
    Plus@2..3 "+"
    Whitespace@3..4 " "
    InfixExpr@4..9
      VariableRef@4..6
        Identifier@4..5 "b"
        Whitespace@5..6 " "
      Plus@6..7 "+"
      Whitespace@7..8 " "
      Literal@8..9
        Number@8..9 "1""#]],
        )
    }
}
