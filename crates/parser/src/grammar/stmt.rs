use super::{CompletedMarker, Parser, expr};
use syntax::{SyntaxConstruct, TokenKind};

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::IfKw) {
        Some(if_stmt(p))
    } else if p.at(TokenKind::BreakKw) {
        Some(break_stmt(p))
    } else if p.at(TokenKind::ContinueKw) {
        Some(continue_stmt(p))
    } else {
        expr::expr(p)
    }
}

fn if_stmt(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(); // if

    // Parse condition
    expr::expr(p);

    // Parse body and branches until endif
    loop {
        if p.at(TokenKind::ElseIfKw) {
            p.bump();
            expr::expr(p);
            continue;
        }
        if p.at(TokenKind::ElseKw) {
            p.bump();
            continue;
        }
        if p.at(TokenKind::EndIfKw) || p.at(TokenKind::EndKw) {
            p.bump();
            break;
        }
        if p.at_end() {
            break;
        }
        // Body statement
        stmt(p);
    }

    m.complete(p, SyntaxConstruct::IfStmt.into())
}

fn break_stmt(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxConstruct::BreakStmt.into())
}

fn continue_stmt(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxConstruct::ContinueStmt.into())
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
                  InfixExpr@0..9
                    VariableRef@0..4
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
                  InfixExpr@0..7
                    VariableRef@0..2
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
                  InfixExpr@0..8
                    VariableRef@0..2
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
                  InfixExpr@0..10
                    VariableRef@0..2
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
                  InfixExpr@0..10
                    VariableRef@0..2
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
                  InfixExpr@10..15
                    VariableRef@10..12
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
                  InfixExpr@0..6
                    VariableRef@0..2
                      Identifier@0..1 "a"
                      Whitespace@1..2 " "
                    Equals@2..3 "="
                    Whitespace@3..4 " "
                    Semicolon@4..5 ";"
                    Newline@5..6 "\n"
                  InfixExpr@6..11
                    VariableRef@6..8
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
                  InfixExpr@0..6
                    VariableRef@0..2
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
                    InfixExpr@0..6
                      VariableRef@0..2
                        Identifier@0..1 "a"
                        Whitespace@1..2 " "
                      Plus@2..3 "+"
                      Whitespace@3..4 " "
                      VariableRef@4..6
                        Identifier@4..5 "b"
                        Whitespace@5..6 " "
                    Plus@6..7 "+"
                    Whitespace@7..8 " "
                    Literal@8..9
                      Number@8..9 "1""#]],
        )
    }

    #[test]
    fn parse_break() {
        check("break", expect![[r#"
Root@0..5
  BreakStmt@0..5
    BreakKw@0..5 "break""#]]);
    }

    #[test]
    fn parse_continue() {
        check("continue", expect![[r#"
Root@0..8
  ContinueStmt@0..8
    ContinueKw@0..8 "continue""#]]);
    }

    #[test]
    fn parse_simple_if() {
        check("if x y endif", expect![[r#"
            Root@0..12
              IfStmt@0..12
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                VariableRef@3..5
                  Identifier@3..4 "x"
                  Whitespace@4..5 " "
                VariableRef@5..7
                  Identifier@5..6 "y"
                  Whitespace@6..7 " "
                EndIfKw@7..12 "endif""#]]);
    }

    #[test]
    fn parse_if_else() {
        check("if x; else; y; endif", expect![[r#"
            Root@0..20
              IfStmt@0..20
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                VariableRef@3..4
                  Identifier@3..4 "x"
                Semicolon@4..5 ";"
                Whitespace@5..6 " "
                ElseKw@6..10 "else"
                Semicolon@10..11 ";"
                Whitespace@11..12 " "
                VariableRef@12..13
                  Identifier@12..13 "y"
                Semicolon@13..14 ";"
                Whitespace@14..15 " "
                EndIfKw@15..20 "endif""#]]);
    }

    #[test]
    fn parse_if_elseif_else() {
        check("if x; elseif y; else; z; endif", expect![[r#"
            Root@0..30
              IfStmt@0..30
                IfKw@0..2 "if"
                Whitespace@2..3 " "
                VariableRef@3..4
                  Identifier@3..4 "x"
                Semicolon@4..5 ";"
                Whitespace@5..6 " "
                ElseIfKw@6..12 "elseif"
                Whitespace@12..13 " "
                VariableRef@13..14
                  Identifier@13..14 "y"
                Semicolon@14..15 ";"
                Whitespace@15..16 " "
                ElseKw@16..20 "else"
                Semicolon@20..21 ";"
                Whitespace@21..22 " "
                VariableRef@22..23
                  Identifier@22..23 "z"
                Semicolon@23..24 ";"
                Whitespace@24..25 " "
                EndIfKw@25..30 "endif""#]]);
    }
}
