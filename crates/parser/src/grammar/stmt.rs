use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    match p.peek() {
        Some(SyntaxKind::Identifier) => variable_def(p).or_else(|| expr::expr(p)),
        _ => expr::expr(p),
    }
}

fn variable_def(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(SyntaxKind::Identifier));
    let m = p.start();
    p.bump();

    if !p.at(SyntaxKind::Equals) {
        return Some(m.complete(p, SyntaxKind::VariableRef));
    }

    p.bump();

    expr::expr(p)?;

    Some(m.complete(p, SyntaxKind::VariableDef))
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
    fn recover_on_semicolon() {
        check(
            "a = ;\nb = a",
            expect![[r#"
Root@0..11
  VariableDef@0..8
    Ident@0..1 "a"
    Whitespace@1..2 " "
    Equals@2..3 "="
    Whitespace@3..4 " "
    Semicolon@4..5 ";"
    Whitespace@5..6 "\n"
  VariableDef@6..17
    Ident@6..7 "b"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    VariableRef@10..11
      Ident@10..11 "a""#]],
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
}
