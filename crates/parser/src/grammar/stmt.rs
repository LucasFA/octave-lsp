use super::{CompletedMarker, Parser, expr};
use syntax::{SyntaxConstruct, TokenKind};

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::FnKw) {
        Some(fn_def(p))
    } else if p.at(TokenKind::IfKw) {
        Some(if_stmt(p))
    } else if p.at(TokenKind::ForKw) {
        Some(for_loop(p))
    } else if p.at(TokenKind::WhileKw) {
        Some(while_loop(p))
    } else if p.at(TokenKind::SwitchKw) {
        Some(switch_stmt(p))
    } else if p.at(TokenKind::TryKw) {
        Some(try_catch_stmt(p))
    } else if p.at(TokenKind::UnwindProtectKw) {
        Some(unwind_protect_stmt(p))
    } else if p.at(TokenKind::BreakKw) {
        Some(break_stmt(p))
    } else if p.at(TokenKind::ContinueKw) {
        Some(continue_stmt(p))
    } else {
        expr::expr(p)
    }
}

fn fn_def(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(); // function

    // Parse the function header (output args = name(input args))
    // e.g. `y = f(x)` or `[out1, out2] = name(in1, in2)`
    expr::expr(p);

    // Parse body statements until endfunction/endfn/end
    loop {
        if p.at(TokenKind::EndFnKw) || p.at(TokenKind::EndKw) {
            p.bump();
            break;
        }
        if p.at_end() {
            break;
        }
        stmt(p);
    }

    m.complete(p, SyntaxConstruct::FnDef.into())
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

fn for_loop(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(); // for

    // Parse loop variable + range: `i = 1:10`
    expr::expr(p);

    // Parse body until endfor/end
    loop {
        if p.at(TokenKind::EndForKw) || p.at(TokenKind::EndKw) {
            p.bump();
            break;
        }
        if p.at_end() {
            break;
        }
        stmt(p);
    }

    m.complete(p, SyntaxConstruct::ForLoop.into())
}

fn while_loop(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(); // while

    // Parse condition
    expr::expr(p);

    // Parse body until endwhile/end
    loop {
        if p.at(TokenKind::EndWhileKw) || p.at(TokenKind::EndKw) {
            p.bump();
            break;
        }
        if p.at_end() {
            break;
        }
        stmt(p);
    }

    m.complete(p, SyntaxConstruct::WhileLoop.into())
}

fn switch_stmt(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(); // switch

    // Parse switched expression
    expr::expr(p);

    // Parse cases until endswitch/end
    loop {
        if p.at(TokenKind::CaseKw) {
            p.bump();
            expr::expr(p); // case value (optional)
            continue;
        }
        if p.at(TokenKind::OtherwiseKw) {
            p.bump();
            continue;
        }
        if p.at(TokenKind::EndSwitchKw) || p.at(TokenKind::EndKw) {
            p.bump();
            break;
        }
        if p.at_end() {
            break;
        }
        stmt(p);
    }

    m.complete(p, SyntaxConstruct::SwitchStmt.into())
}

fn try_catch_stmt(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(); // try

    // Parse body until catch or end_try_catch
    loop {
        if p.at(TokenKind::CatchKw) {
            p.bump();
            continue;
        }
        if p.at(TokenKind::EndTryKw) || p.at(TokenKind::EndKw) {
            p.bump();
            break;
        }
        if p.at_end() {
            break;
        }
        stmt(p);
    }

    m.complete(p, SyntaxConstruct::TryStmt.into())
}

fn unwind_protect_stmt(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(); // unwind_protect

    // Parse body until cleanup or end_unwind_protect
    loop {
        if p.at(TokenKind::UnwindProtectCleanupKw) {
            p.bump();
            continue;
        }
        if p.at(TokenKind::EndUnwindProtectKw) || p.at(TokenKind::EndKw) {
            p.bump();
            break;
        }
        if p.at_end() {
            break;
        }
        stmt(p);
    }

    m.complete(p, SyntaxConstruct::UnwindProtectStmt.into())
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
        check(
            "break",
            expect![[r#"
Root@0..5
  BreakStmt@0..5
    BreakKw@0..5 "break""#]],
        );
    }

    #[test]
    fn parse_continue() {
        check(
            "continue",
            expect![[r#"
Root@0..8
  ContinueStmt@0..8
    ContinueKw@0..8 "continue""#]],
        );
    }

    #[test]
    fn parse_simple_if() {
        check(
            "if x y endif",
            expect![[r#"
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
                EndIfKw@7..12 "endif""#]],
        );
    }

    #[test]
    fn parse_if_else() {
        check(
            "if x; else; y; endif",
            expect![[r#"
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
                EndIfKw@15..20 "endif""#]],
        );
    }

    #[test]
    fn parse_if_elseif_else() {
        check(
            "if x; elseif y; else; z; endif",
            expect![[r#"
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
    EndIfKw@25..30 "endif""#]],
        );
    }

    #[test]
    fn parse_for_loop() {
        check(
            "for i = 1:10; x; endfor",
            expect![[r#"
            Root@0..23
              ForLoop@0..23
                ForKw@0..3 "for"
                Whitespace@3..4 " "
                InfixExpr@4..14
                  VariableRef@4..6
                    Identifier@4..5 "i"
                    Whitespace@5..6 " "
                  Equals@6..7 "="
                  Whitespace@7..8 " "
                  InfixExpr@8..14
                    Literal@8..9
                      Number@8..9 "1"
                    Colon@9..10 ":"
                    Literal@10..12
                      Number@10..12 "10"
                    Semicolon@12..13 ";"
                    Whitespace@13..14 " "
                VariableRef@14..15
                  Identifier@14..15 "x"
                Semicolon@15..16 ";"
                Whitespace@16..17 " "
                EndForKw@17..23 "endfor""#]],
        );
    }

    #[test]
    fn parse_while_loop() {
        check(
            "while x; y; endwhile",
            expect![[r#"
            Root@0..20
              WhileLoop@0..20
                WhileKw@0..5 "while"
                Whitespace@5..6 " "
                VariableRef@6..7
                  Identifier@6..7 "x"
                Semicolon@7..8 ";"
                Whitespace@8..9 " "
                VariableRef@9..10
                  Identifier@9..10 "y"
                Semicolon@10..11 ";"
                Whitespace@11..12 " "
                EndWhileKw@12..20 "endwhile""#]],
        );
    }

    #[test]
    fn parse_switch() {
        check(
            "switch x; case 1; y; otherwise; z; endswitch",
            expect![[r#"
Root@0..44
  SwitchStmt@0..44
    SwitchKw@0..6 "switch"
    Whitespace@6..7 " "
    VariableRef@7..8
      Identifier@7..8 "x"
    Semicolon@8..9 ";"
    Whitespace@9..10 " "
    CaseKw@10..14 "case"
    Whitespace@14..15 " "
    Literal@15..16
      Number@15..16 "1"
    Semicolon@16..17 ";"
    Whitespace@17..18 " "
    VariableRef@18..19
      Identifier@18..19 "y"
    Semicolon@19..20 ";"
    Whitespace@20..21 " "
    OtherwiseKw@21..30 "otherwise"
    Semicolon@30..31 ";"
    Whitespace@31..32 " "
    VariableRef@32..33
      Identifier@32..33 "z"
    Semicolon@33..34 ";"
    Whitespace@34..35 " "
    EndSwitchKw@35..44 "endswitch""#]],
        );
    }

    #[test]
    fn parse_function_def() {
        check(
            "function y = f(x)\n  y = x * 2\nend",
            expect![[r#"
            Root@0..33
              FnDef@0..33
                FnKw@0..8 "function"
                Whitespace@8..9 " "
                InfixExpr@9..20
                  VariableRef@9..11
                    Identifier@9..10 "y"
                    Whitespace@10..11 " "
                  Equals@11..12 "="
                  Whitespace@12..13 " "
                  CallExpr@13..20
                    VariableRef@13..14
                      Identifier@13..14 "f"
                    LParen@14..15 "("
                    VariableRef@15..16
                      Identifier@15..16 "x"
                    RParen@16..17 ")"
                    Newline@17..18 "\n"
                    Whitespace@18..20 "  "
                InfixExpr@20..30
                  VariableRef@20..22
                    Identifier@20..21 "y"
                    Whitespace@21..22 " "
                  Equals@22..23 "="
                  Whitespace@23..24 " "
                  InfixExpr@24..30
                    VariableRef@24..26
                      Identifier@24..25 "x"
                      Whitespace@25..26 " "
                    Asterisk@26..27 "*"
                    Whitespace@27..28 " "
                    Literal@28..30
                      Number@28..29 "2"
                      Newline@29..30 "\n"
                EndKw@30..33 "end""#]],
        );
    }

    #[test]
    fn parse_try_catch() {
        check(
            "try\n  x = 1\ncatch\n  x = 2\nend_try_catch",
            expect![[r#"
                Root@0..39
                  TryStmt@0..39
                    TryKw@0..3 "try"
                    Newline@3..4 "\n"
                    Whitespace@4..6 "  "
                    InfixExpr@6..12
                      VariableRef@6..8
                        Identifier@6..7 "x"
                        Whitespace@7..8 " "
                      Equals@8..9 "="
                      Whitespace@9..10 " "
                      Literal@10..12
                        Number@10..11 "1"
                        Newline@11..12 "\n"
                    CatchKw@12..17 "catch"
                    Newline@17..18 "\n"
                    Whitespace@18..20 "  "
                    InfixExpr@20..26
                      VariableRef@20..22
                        Identifier@20..21 "x"
                        Whitespace@21..22 " "
                      Equals@22..23 "="
                      Whitespace@23..24 " "
                      Literal@24..26
                        Number@24..25 "2"
                        Newline@25..26 "\n"
                    EndTryKw@26..39 "end_try_catch""#]],
        );
    }

    #[test]
    fn parse_unwind_protect() {
        check(
            "unwind_protect\n  x = 1\nunwind_protect_cleanup\n  x = 2\nend_unwind_protect",
            expect![[r#"
                Root@0..72
                  UnwindProtectStmt@0..72
                    UnwindProtectKw@0..14 "unwind_protect"
                    Newline@14..15 "\n"
                    Whitespace@15..17 "  "
                    InfixExpr@17..23
                      VariableRef@17..19
                        Identifier@17..18 "x"
                        Whitespace@18..19 " "
                      Equals@19..20 "="
                      Whitespace@20..21 " "
                      Literal@21..23
                        Number@21..22 "1"
                        Newline@22..23 "\n"
                    UnwindProtectCleanupKw@23..45 "unwind_protect_cleanup"
                    Newline@45..46 "\n"
                    Whitespace@46..48 "  "
                    InfixExpr@48..54
                      VariableRef@48..50
                        Identifier@48..49 "x"
                        Whitespace@49..50 " "
                      Equals@50..51 "="
                      Whitespace@51..52 " "
                      Literal@52..54
                        Number@52..53 "2"
                        Newline@53..54 "\n"
                    EndUnwindProtectKw@54..72 "end_unwind_protect""#]],
        );
    }
}
