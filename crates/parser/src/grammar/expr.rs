///a Parsing of expressions.
///a This module contains the code for parsing __expressions__.
use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use syntax::SyntaxKind;

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum UnaryOp {
    Neg,
}

impl UnaryOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_binding_power(p, 0)
}

fn literal(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Number));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn variable_ref(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Identifier));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VariableRef)
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Minus));

    let m = p.start();

    let op = UnaryOp::Neg;
    let ((), right_binding_power) = op.binding_power();

    // Eat the operator’s token.
    p.bump();

    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::LParen));

    let m = p.start();

    p.bump();
    expr_binding_power(p, 0);

    assert!(p.at(SyntaxKind::RParen));
    p.bump();

    m.complete(p, SyntaxKind::ParenExpr)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = match p.peek() {
        Some(SyntaxKind::Number) => literal(p),
        Some(SyntaxKind::Identifier) => variable_ref(p),
        Some(SyntaxKind::Minus) => prefix_expr(p),
        Some(SyntaxKind::LParen) => paren_expr(p),
        _ => {
            p.error();
            return None;
        }
    };

    Some(cm)
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) -> Option<CompletedMarker> {
    let mut lhs = lhs(p)?;

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => BinaryOp::Add,
            Some(SyntaxKind::Minus) => BinaryOp::Sub,
            Some(SyntaxKind::Asterisk) => BinaryOp::Mul,
            Some(SyntaxKind::Slash) => BinaryOp::Div,
            _ => break, // we’ll handle errors later.
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }
        p.bump();

        let m = lhs.precede(p);
        expr_binding_power(p, right_binding_power);
        lhs = m.complete(p, SyntaxKind::InfixExpr);
    }

    Some(lhs)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_identifier() {
        check(
            "hello",
            expect![[r#"
                Root@0..5
                  VariableRef@0..5
                    Identifier@0..5 "hello""#]],
        );
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
                Root@0..3
                  Literal@0..3
                    Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1+2",
            expect![[r#"
                Root@0..3
                  InfixExpr@0..3
                    Literal@0..1
                      Number@0..1 "1"
                    Plus@1..2 "+"
                    Literal@2..3
                      Number@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_binary_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    InfixExpr@0..5
                      InfixExpr@0..3
                        Literal@0..1
                          Number@0..1 "1"
                        Plus@1..2 "+"
                        Literal@2..3
                          Number@2..3 "2"
                      Plus@3..4 "+"
                      Literal@4..5
                        Number@4..5 "3"
                    Plus@5..6 "+"
                    Literal@6..7
                      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    InfixExpr@0..5
                      Literal@0..1
                        Number@0..1 "1"
                      Plus@1..2 "+"
                      InfixExpr@2..5
                        Literal@2..3
                          Number@2..3 "2"
                        Asterisk@3..4 "*"
                        Literal@4..5
                          Number@4..5 "3"
                    Minus@5..6 "-"
                    Literal@6..7
                      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-1",
            expect![[r#"
                Root@0..2
                  PrefixExpr@0..2
                    Minus@0..1 "-"
                    Literal@1..2
                      Number@1..2 "1""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_infix_operators() {
        check(
            "-20+20",
            expect![[r#"
                Root@0..6
                  InfixExpr@0..6
                    PrefixExpr@0..3
                      Minus@0..1 "-"
                      Literal@1..3
                        Number@1..3 "20"
                    Plus@3..4 "+"
                    Literal@4..6
                      Number@4..6 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
                Root@0..14
                  ParenExpr@0..14
                    LParen@0..1 "("
                    ParenExpr@1..13
                      LParen@1..2 "("
                      ParenExpr@2..12
                        LParen@2..3 "("
                        ParenExpr@3..11
                          LParen@3..4 "("
                          ParenExpr@4..10
                            LParen@4..5 "("
                            ParenExpr@5..9
                              LParen@5..6 "("
                              Literal@6..8
                                Number@6..8 "10"
                              RParen@8..9 ")"
                            RParen@9..10 ")"
                          RParen@10..11 ")"
                        RParen@11..12 ")"
                      RParen@12..13 ")"
                    RParen@13..14 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
                Root@0..7
                  InfixExpr@0..7
                    Literal@0..1
                      Number@0..1 "5"
                    Asterisk@1..2 "*"
                    ParenExpr@2..7
                      LParen@2..3 "("
                      InfixExpr@3..6
                        Literal@3..4
                          Number@3..4 "2"
                        Plus@4..5 "+"
                        Literal@5..6
                          Number@5..6 "1"
                      RParen@6..7 ")""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
                Root@0..7
                  Whitespace@0..3 "   "
                  Literal@3..7
                    Number@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
                Root@0..6
                  Literal@0..6
                    Number@0..3 "999"
                    Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
                Root@0..9
                  Whitespace@0..1 " "
                  Literal@1..9
                    Number@1..4 "123"
                    Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
                Root@0..12
                  Whitespace@0..1 " "
                  InfixExpr@1..12
                    Literal@1..3
                      Number@1..2 "1"
                      Whitespace@2..3 " "
                    Plus@3..4 "+"
                    Whitespace@4..7 "   "
                    InfixExpr@7..12
                      Literal@7..8
                        Number@7..8 "2"
                      Asterisk@8..9 "*"
                      Whitespace@9..10 " "
                      Literal@10..12
                        Number@10..11 "3"
                        Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn parse_binary_expression_followed_by_comments() {
        check(
            "1 + 1+ 10 # Add ten",
            expect![[r##"
Root@0..19
  InfixExpr@0..19
    InfixExpr@0..5
      Literal@0..2
        Number@0..1 "1"
        Whitespace@1..2 " "
      Plus@2..3 "+"
      Whitespace@3..4 " "
      Literal@4..5
        Number@4..5 "1"
    Plus@5..6 "+"
    Whitespace@6..7 " "
    Literal@7..19
      Number@7..9 "10"
      Whitespace@9..10 " "
      Comment@10..19 "# Add ten""##]],
        );
    }

    //     #[test]
    //     fn parse_whitespace_and_identifier() {
    //         check(
    //             "he + llo ",
    //             expect![[r#"
    // Root@0..7
    //   Identifier@0..2 "he"
    //   Whitespace@2..3 " "
    //   Identifier@3..6 "llo""#]],
    //         );
    //     }
}
