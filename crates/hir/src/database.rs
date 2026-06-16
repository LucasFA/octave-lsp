use crate::{BinaryOp, Expr, Stmt, TransposeOp, UnaryOp};
use la_arena::Arena;
use syntax::{SyntaxKind, TokenKind};

#[derive(Debug, PartialEq, Default)]
pub struct Database {
    exprs: Arena<Expr>,
}

impl Database {
    pub(crate) fn lower_stmt(&mut self, ast: ast::Stmt) -> Option<Stmt> {
        let result = match ast {
            ast::Stmt::VariableDef(ast) => Stmt::VariableDef {
                name: ast.name()?.text().into(),
                value: self.lower_expr(ast.value()),
            },
            ast::Stmt::FnDef(ast) => Stmt::FnDef {
                body: ast.body().filter_map(|s| self.lower_stmt(s)).collect(),
            },
            ast::Stmt::IfStmt(ast) => {
                let condition = self.lower_expr(ast.condition());
                Stmt::If {
                    condition: self.exprs.alloc(condition),
                    body: ast.body().filter_map(|s| self.lower_stmt(s)).collect(),
                }
            }
            ast::Stmt::ForLoop(ast) => Stmt::ForLoop {
                body: ast.body().filter_map(|s| self.lower_stmt(s)).collect(),
            },
            ast::Stmt::WhileLoop(ast) => {
                let condition = self.lower_expr(ast.condition());
                Stmt::WhileLoop {
                    condition: self.exprs.alloc(condition),
                    body: ast.body().filter_map(|s| self.lower_stmt(s)).collect(),
                }
            }
            ast::Stmt::BreakStmt(_) => Stmt::Break,
            ast::Stmt::ContinueStmt(_) => Stmt::Continue,
            ast::Stmt::TryStmt(ast) => Stmt::Try {
                body: ast.body().filter_map(|s| self.lower_stmt(s)).collect(),
                catch: vec![], // TODO: split catch body from try body by CatchKw token
            },
            ast::Stmt::UnwindProtectStmt(ast) => Stmt::UnwindProtect {
                body: ast.body().filter_map(|s| self.lower_stmt(s)).collect(),
                cleanup: vec![], // TODO: split cleanup body by UnwindProtectCleanupKw token
            },
            ast::Stmt::SwitchStmt(ast) => {
                let condition = self.lower_expr(ast.condition());
                Stmt::Switch {
                    condition: self.exprs.alloc(condition),
                    body: ast.body().filter_map(|s| self.lower_stmt(s)).collect(),
                }
            }
            ast::Stmt::Expr(ast) => Stmt::Expr(self.lower_expr(Some(ast))),
        };

        Some(result)
    }

    pub(crate) fn lower_expr(&mut self, ast: Option<ast::Expr>) -> Expr {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => self.lower_binary(&ast),
                ast::Expr::Literal(ast) => Expr::Literal { n: ast.parse() },
                ast::Expr::ParenExpr(ast) => self.lower_expr(ast.expr()),
                ast::Expr::UnaryExpr(ast) => self.lower_unary(&ast),
                ast::Expr::VariableRef(ast) => Database::lower_variable_ref(&ast),
                ast::Expr::MatrixExpr(ast) => self.lower_matrix(&ast),
                ast::Expr::CallExpr(ast) => self.lower_call(&ast),
                ast::Expr::PostfixExpr(ast) => self.lower_postfix(&ast),
                ast::Expr::StringLiteral(ast) => Expr::String {
                    value: ast.value().into(),
                },
            }
        } else {
            Expr::Missing
        }
    }

    fn lower_variable_ref(ast: &ast::VariableRef) -> Expr {
        Expr::VariableRef {
            var: ast.name().unwrap().text().into(),
        }
    }

    fn lower_binary(&mut self, ast: &ast::BinaryExpr) -> Expr {
        let SyntaxKind::LexToken(token_kind) = ast.op().unwrap().kind() else {
            unreachable!()
        };

        let lhs = self.lower_expr(ast.lhs());
        let rhs = self.lower_expr(ast.rhs());

        match token_kind {
            TokenKind::Colon => Expr::Range {
                lhs: self.exprs.alloc(lhs),
                rhs: self.exprs.alloc(rhs),
            },
            kind => {
                let op = match kind {
                    TokenKind::Plus => BinaryOp::Add,
                    TokenKind::Minus => BinaryOp::Sub,
                    TokenKind::Asterisk => BinaryOp::Mul,
                    TokenKind::Slash => BinaryOp::Div,
                    TokenKind::ElmtMult => BinaryOp::ElmtMult,
                    TokenKind::ElmtDiv => BinaryOp::ElmtDiv,
                    TokenKind::LeftDiv => BinaryOp::LeftDiv,
                    TokenKind::ElmtLeftDiv => BinaryOp::ElmtLeftDiv,
                    TokenKind::Caret => BinaryOp::Pow,
                    TokenKind::ElmtPow => BinaryOp::ElmtPow,
                    TokenKind::EqualsEquals => BinaryOp::EqualsEquals,
                    TokenKind::NotEquals | TokenKind::TildeEquals => BinaryOp::NotEquals,
                    TokenKind::LessThan => BinaryOp::LessThan,
                    TokenKind::GreaterThan => BinaryOp::GreaterThan,
                    TokenKind::LessThanEquals => BinaryOp::LessThanEquals,
                    TokenKind::GreaterThanEquals => BinaryOp::GreaterThanEquals,
                    TokenKind::And => BinaryOp::And,
                    TokenKind::Or => BinaryOp::Or,
                    TokenKind::Equals => BinaryOp::Assign,
                    TokenKind::PlusEquals => BinaryOp::PlusEquals,
                    TokenKind::MinusEquals => BinaryOp::MinusEquals,
                    TokenKind::AsteriskEquals => BinaryOp::AsteriskEquals,
                    TokenKind::SlashEquals => BinaryOp::SlashEquals,
                    TokenKind::ElmtMultEquals => BinaryOp::ElmtMultEquals,
                    TokenKind::ElmtDivEquals => BinaryOp::ElmtDivEquals,
                    TokenKind::ElmtPowEquals => BinaryOp::ElmtPowEquals,
                    _ => unreachable!(),
                };
                Expr::Binary {
                    op,
                    lhs: self.exprs.alloc(lhs),
                    rhs: self.exprs.alloc(rhs),
                }
            }
        }
    }

    fn lower_unary(&mut self, ast: &ast::UnaryExpr) -> Expr {
        let SyntaxKind::LexToken(token_kind) = ast.op().unwrap().kind() else {
            unreachable!()
        };

        let op = match token_kind {
            TokenKind::Minus => UnaryOp::Neg,
            TokenKind::Plus => UnaryOp::Plus,
            TokenKind::Not => UnaryOp::Not,
            TokenKind::Tilde => UnaryOp::Tilde,
            _ => unreachable!(),
        };

        let expr = self.lower_expr(ast.expr());

        Expr::Unary {
            op,
            expr: self.exprs.alloc(expr),
        }
    }

    fn lower_call(&mut self, ast: &ast::CallExpr) -> Expr {
        let func = self.lower_expr(ast.func());
        let args: Vec<_> = ast.args().map(|e| self.lower_expr(Some(e))).collect();
        Expr::Call {
            func: self.exprs.alloc(func),
            args: args.into_iter().map(|e| self.exprs.alloc(e)).collect(),
        }
    }

    fn lower_matrix(&mut self, ast: &ast::MatrixExpr) -> Expr {
        let elements: Vec<_> = ast.elements().map(|e| self.lower_expr(Some(e))).collect();
        Expr::Matrix {
            elements: elements.into_iter().map(|e| self.exprs.alloc(e)).collect(),
        }
    }

    fn lower_postfix(&mut self, ast: &ast::PostfixExpr) -> Expr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::LexToken(TokenKind::Transpose) => TransposeOp::Normal,
            SyntaxKind::LexToken(TokenKind::ElmtTranspose) => TransposeOp::Elmt,
            _ => unreachable!(),
        };
        let expr = self.lower_expr(ast.expr());
        Expr::Transpose {
            op,
            expr: self.exprs.alloc(expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::TypedSyntaxNode;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check_stmt(input: &str, expected_hir: Stmt) {
        let root = parse(input);
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(hir, expected_hir);
    }

    fn check_expr(input: &str, expected_hir: Expr, expected_database: Database) {
        let root = parse(input);
        let first_stmt = root.stmts().next().unwrap();
        let ast = match first_stmt {
            ast::Stmt::Expr(ast) => ast,
            _ => unreachable!(),
        };
        let mut database = Database::default();
        let hir = database.lower_expr(Some(ast));

        assert_eq!(hir, expected_hir);
        assert_eq!(database, expected_database);
    }

    #[test]
    fn lower_variable_def() {
        let root = parse("foo = bar");
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(
            hir,
            Stmt::VariableDef {
                name: "foo".into(),
                value: Expr::VariableRef { var: "bar".into() },
            },
        );
    }

    #[test]
    fn lower_expr_stmt() {
        check_stmt("123", Stmt::Expr(Expr::Literal { n: Some(123) }));
    }

    #[test]
    fn lower_binary_expr() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: Some(1) });
        let rhs = exprs.alloc(Expr::Literal { n: Some(2) });

        check_expr(
            "1 + 2",
            Expr::Binary {
                lhs,
                rhs,
                op: BinaryOp::Add,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_literal() {
        check_expr("999", Expr::Literal { n: Some(999) }, Database::default());
    }

    #[test]
    fn lower_paren_expr() {
        check_expr(
            "((((((abc))))))",
            Expr::VariableRef { var: "abc".into() },
            Database::default(),
        );
    }

    #[test]
    fn lower_unary_expr() {
        let mut exprs = Arena::new();
        let ten = exprs.alloc(Expr::Literal { n: Some(10) });

        check_expr(
            "-10",
            Expr::Unary {
                expr: ten,
                op: UnaryOp::Neg,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_variable_ref() {
        check_expr(
            "foo",
            Expr::VariableRef { var: "foo".into() },
            Database::default(),
        );
    }

    #[test]
    fn lower_variable_def_without_value() {
        check_stmt(
            "a =",
            Stmt::VariableDef {
                name: "a".into(),
                value: Expr::Missing,
            },
        );
    }

    #[test]
    fn lower_binary_expr_without_rhs() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: Some(10) });
        let rhs = exprs.alloc(Expr::Missing);

        check_expr(
            "10 -",
            Expr::Binary {
                lhs,
                rhs,
                op: BinaryOp::Sub,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_unary_expr_without_expr() {
        let mut exprs = Arena::new();
        let expr = exprs.alloc(Expr::Missing);

        check_expr(
            "-",
            Expr::Unary {
                expr,
                op: UnaryOp::Neg,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_binary_elmt_mult() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: Some(2) });
        let rhs = exprs.alloc(Expr::Literal { n: Some(3) });

        check_expr(
            "2 .* 3",
            Expr::Binary {
                lhs,
                rhs,
                op: BinaryOp::ElmtMult,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_binary_pow() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: Some(2) });
        let rhs = exprs.alloc(Expr::Literal { n: Some(3) });

        check_expr(
            "2 ^ 3",
            Expr::Binary {
                lhs,
                rhs,
                op: BinaryOp::Pow,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_range() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(Expr::Literal { n: Some(1) });
        let rhs = exprs.alloc(Expr::Literal { n: Some(10) });

        check_expr("1:10", Expr::Range { lhs, rhs }, Database { exprs });
    }

    #[test]
    fn lower_call_no_args() {
        let mut exprs = Arena::new();
        let func = exprs.alloc(Expr::VariableRef { var: "f".into() });

        check_expr("f()", Expr::Call { func, args: vec![] }, Database { exprs });
    }

    #[test]
    fn lower_call_one_arg() {
        let mut exprs = Arena::new();
        let func = exprs.alloc(Expr::VariableRef { var: "f".into() });
        let arg = exprs.alloc(Expr::Literal { n: Some(42) });

        check_expr(
            "f(42)",
            Expr::Call {
                func,
                args: vec![arg],
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_matrix_2d() {
        let mut exprs = Arena::new();
        let e1 = exprs.alloc(Expr::Literal { n: Some(1) });
        let e2 = exprs.alloc(Expr::Literal { n: Some(2) });
        let e3 = exprs.alloc(Expr::Literal { n: Some(3) });
        let e4 = exprs.alloc(Expr::Literal { n: Some(4) });

        check_expr(
            "[1, 2; 3, 4]",
            Expr::Matrix {
                elements: vec![e1, e2, e3, e4],
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_if_stmt() {
        let mut exprs = Arena::new();
        let condition = exprs.alloc(Expr::VariableRef { var: "x".into() });

        check_stmt(
            "if x\n  y\nendif",
            Stmt::If {
                condition,
                body: vec![Stmt::Expr(Expr::VariableRef { var: "y".into() })],
            },
        );
    }

    #[test]
    fn lower_while_loop() {
        let mut exprs = Arena::new();
        let condition = exprs.alloc(Expr::Literal { n: Some(1) });
        let x_ref = exprs.alloc(Expr::VariableRef { var: "x".into() });

        check_stmt(
            "while 1\n  x = x + 1\nendwhile",
            Stmt::WhileLoop {
                condition,
                body: vec![Stmt::VariableDef {
                    name: "x".into(),
                    value: Expr::Binary {
                        op: BinaryOp::Add,
                        lhs: x_ref,
                        rhs: exprs.alloc(Expr::Literal { n: Some(1) }),
                    },
                }],
            },
        );
    }

    #[test]
    fn lower_break() {
        check_stmt("break", Stmt::Break);
    }

    #[test]
    fn lower_continue() {
        check_stmt("continue", Stmt::Continue);
    }

    #[test]
    fn lower_transpose() {
        let mut exprs = Arena::new();
        let var = exprs.alloc(Expr::VariableRef { var: "a".into() });

        check_expr(
            "a'",
            Expr::Transpose {
                op: TransposeOp::Normal,
                expr: var,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_elmt_transpose() {
        let mut exprs = Arena::new();
        let var = exprs.alloc(Expr::VariableRef { var: "a".into() });

        check_expr(
            "a.'",
            Expr::Transpose {
                op: TransposeOp::Elmt,
                expr: var,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_try_catch() {
        check_stmt(
            "try\n  x = 1\ncatch\n  x = 2\nend_try_catch",
            Stmt::Try {
                body: vec![
                    Stmt::VariableDef {
                        name: "x".into(),
                        value: Expr::Literal { n: Some(1) },
                    },
                    Stmt::VariableDef {
                        name: "x".into(),
                        value: Expr::Literal { n: Some(2) },
                    },
                ],
                catch: vec![],
            },
        );
    }

    #[test]
    fn lower_unwind_protect() {
        check_stmt(
            "unwind_protect\n  x = 1\nunwind_protect_cleanup\n  x = 2\nend_unwind_protect",
            Stmt::UnwindProtect {
                body: vec![
                    Stmt::VariableDef {
                        name: "x".into(),
                        value: Expr::Literal { n: Some(1) },
                    },
                    Stmt::VariableDef {
                        name: "x".into(),
                        value: Expr::Literal { n: Some(2) },
                    },
                ],
                cleanup: vec![],
            },
        );
    }

    #[test]
    fn lower_string_literal() {
        check_expr(
            "'hello'",
            Expr::String {
                value: "hello".into(),
            },
            Database::default(),
        );
    }

    #[test]
    fn lower_string_empty() {
        check_expr("''", Expr::String { value: "".into() }, Database::default());
    }

    #[test]
    fn lower_string_escaped_quote() {
        check_expr(
            "'it''s'",
            Expr::String {
                value: "it's".into(),
            },
            Database::default(),
        );
    }
}
