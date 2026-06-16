#![warn(clippy::pedantic)]

use syntax::{SyntaxConstruct, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, TokenKind};
pub mod validation;

pub trait TypedSyntaxNode {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized;
}

#[macro_use]
mod macros {
    macro_rules! impl_typed_syntax_node {
        ($struct_name:ident) => {
            #[allow(dead_code)]
            #[derive(Debug)]
            pub struct $struct_name(SyntaxNode);
            impl TypedSyntaxNode for $struct_name {
                fn cast(node: SyntaxNode) -> Option<Self>
                where
                    Self: Sized,
                {
                    if let SyntaxKind::SyntaxConstruct(SyntaxConstruct::$struct_name) = node.kind()
                    {
                        Some(Self(node))
                    } else {
                        None
                    }
                }
            }
            const _: () = {
                // Simply a check that struct_name is a variant of SyntaxConstruct
                const _: SyntaxConstruct = SyntaxConstruct::$struct_name;
            };
        };
    }
}

impl_typed_syntax_node!(VariableRef);
impl_typed_syntax_node!(Root);
impl_typed_syntax_node!(MatrixExpr);
impl_typed_syntax_node!(CallExpr);
impl_typed_syntax_node!(PostfixExpr);
impl_typed_syntax_node!(FnDef);
impl_typed_syntax_node!(IfStmt);
impl_typed_syntax_node!(ForLoop);
impl_typed_syntax_node!(WhileLoop);
impl_typed_syntax_node!(BreakStmt);
impl_typed_syntax_node!(ContinueStmt);
impl_typed_syntax_node!(SwitchStmt);
impl_typed_syntax_node!(TryStmt);
impl_typed_syntax_node!(UnwindProtectStmt);

#[derive(Debug)]
pub struct VariableDef(SyntaxNode);

impl VariableDef {
    #[must_use]
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::InfixExpr) => {
                let is_assign = node
                    .children_with_tokens()
                    .filter_map(SyntaxElement::into_token)
                    .any(|token| token.kind() == SyntaxKind::LexToken(TokenKind::Equals));
                if is_assign {
                    Some(Self(node))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Root {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast)
    }

    pub fn get_variable_definitions(&self) -> impl Iterator<Item = VariableDef> {
        self.stmts().filter_map(|stmt| {
            if let Stmt::VariableDef(var_def) = stmt {
                Some(var_def)
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn get_variable_references(&self) -> Vec<VariableRef> {
        fn inner_get_variable_references(node: SyntaxNode) -> Vec<VariableRef> {
            if node.kind() == SyntaxConstruct::VariableRef.into() {
                vec![VariableRef::cast(node).unwrap()]
            } else {
                node.children()
                    .flat_map(inner_get_variable_references)
                    .collect()
            }
        }
        inner_get_variable_references(self.0.clone())
    }
}

impl VariableDef {
    /// Returns the name of the defined variable, or None if the LHS is not a `VariableRef`.
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxConstruct::VariableRef.into())?
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == TokenKind::Identifier.into())
    }

    /// Returns the value of the defined variable.
    ///
    /// If the variable definition does not have a value, returns None.
    pub fn value(&self) -> Option<Expr> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .filter_map(Expr::cast)
            .nth(1)
    }
}

impl Expr {
    #[must_use]
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result;
        if let SyntaxKind::SyntaxConstruct(inner) = node.kind() {
            result = match inner {
                SyntaxConstruct::InfixExpr => Self::BinaryExpr(BinaryExpr(node)),
                SyntaxConstruct::Literal => Self::Literal(Literal(node)),
                SyntaxConstruct::ParenExpr => Self::ParenExpr(ParenExpr(node)),
                SyntaxConstruct::PrefixExpr => Self::UnaryExpr(UnaryExpr(node)),
                SyntaxConstruct::VariableRef => Self::VariableRef(VariableRef(node)),
                SyntaxConstruct::MatrixExpr => Self::MatrixExpr(MatrixExpr(node)),
                SyntaxConstruct::CallExpr => Self::CallExpr(CallExpr(node)),
                SyntaxConstruct::PostfixExpr => Self::PostfixExpr(PostfixExpr(node)),
                SyntaxConstruct::Root => unreachable!(),
                SyntaxConstruct::Error
                | SyntaxConstruct::Block
                | SyntaxConstruct::FnDef
                | SyntaxConstruct::IfStmt
                | SyntaxConstruct::ForLoop
                | SyntaxConstruct::WhileLoop
                | SyntaxConstruct::BreakStmt
                | SyntaxConstruct::ContinueStmt
                | SyntaxConstruct::SwitchStmt
                | SyntaxConstruct::TryStmt
                | SyntaxConstruct::UnwindProtectStmt => return None,
            };
        } else {
            return None;
        }

        Some(result)
    }
}

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                if let SyntaxKind::LexToken(inner) = token.kind() {
                    matches!(
                        inner,
                        TokenKind::Plus
                            | TokenKind::Minus
                            | TokenKind::Asterisk
                            | TokenKind::Slash
                            | TokenKind::ElmtMult
                            | TokenKind::ElmtDiv
                            | TokenKind::LeftDiv
                            | TokenKind::ElmtLeftDiv
                            | TokenKind::Caret
                            | TokenKind::ElmtPow
                            | TokenKind::Colon
                            | TokenKind::EqualsEquals
                            | TokenKind::NotEquals
                            | TokenKind::LessThan
                            | TokenKind::GreaterThan
                            | TokenKind::LessThanEquals
                            | TokenKind::GreaterThanEquals
                            | TokenKind::TildeEquals
                            | TokenKind::And
                            | TokenKind::Or
                            | TokenKind::Equals
                            | TokenKind::PlusEquals
                            | TokenKind::MinusEquals
                            | TokenKind::AsteriskEquals
                            | TokenKind::SlashEquals
                            | TokenKind::ElmtMultEquals
                            | TokenKind::ElmtDivEquals
                            | TokenKind::ElmtPowEquals
                    )
                } else {
                    false
                }
            })
    }
}

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::LexToken(
                        TokenKind::Minus
                            | TokenKind::Plus
                            | TokenKind::Not
                            | TokenKind::Tilde
                    )
                )
            })
    }
}

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

impl Literal {
    /// Returns the value of the literal as a u64
    ///
    /// # Panics
    ///
    /// Panics if there are no more tokens left
    ///
    /// # Errors
    ///
    /// Returns None if the token is not a valid u64
    #[must_use]
    pub fn parse(&self) -> Option<u64> {
        self.0.first_token().unwrap().text().parse().ok()
    }

    #[must_use]
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxConstruct::Literal.into() {
            Some(Self(node))
        } else {
            None
        }
    }
}

impl VariableRef {
    /// Returns the name of the variable
    ///
    /// # Errors
    ///
    /// Returns None if erronously called on a node that is not a variable reference
    #[must_use]
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }
}

impl CallExpr {
    #[must_use]
    pub fn func(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn args(&self) -> impl Iterator<Item = Expr> {
        self.0.children().filter_map(Expr::cast).skip(1)
    }
}

impl PostfixExpr {
    #[must_use]
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    #[must_use]
    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::LexToken(TokenKind::Transpose | TokenKind::ElmtTranspose)
                )
            })
    }
}

impl FnDef {
    pub fn body(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast).skip(1)
    }
}

impl IfStmt {
    #[must_use]
    pub fn condition(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn body(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast).skip(1)
    }
}

impl WhileLoop {
    #[must_use]
    pub fn condition(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn body(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast).skip(1)
    }
}

impl ForLoop {
    pub fn body(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast).skip(1)
    }
}

impl SwitchStmt {
    #[must_use]
    pub fn condition(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn body(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast).skip(1)
    }
}

impl TryStmt {
    pub fn body(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast)
    }
}

impl UnwindProtectStmt {
    pub fn body(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast)
    }
}

impl MatrixExpr {
    pub fn elements(&self) -> impl Iterator<Item = Expr> {
        self.0.children().filter_map(Expr::cast)
    }
}

#[derive(Debug)]
pub enum Stmt {
    VariableDef(VariableDef),
    FnDef(FnDef),
    IfStmt(IfStmt),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    SwitchStmt(SwitchStmt),
    TryStmt(TryStmt),
    UnwindProtectStmt(UnwindProtectStmt),
    Expr(Expr),
}

impl Stmt {
    #[must_use]
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(var_def) = VariableDef::cast(node.clone()) {
            return Some(Self::VariableDef(var_def));
        }
        let result = match node.kind() {
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::FnDef) => Self::FnDef(FnDef(node)),
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::IfStmt) => Self::IfStmt(IfStmt(node)),
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::ForLoop) => Self::ForLoop(ForLoop(node)),
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::WhileLoop) => Self::WhileLoop(WhileLoop(node)),
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::BreakStmt) => Self::BreakStmt(BreakStmt(node)),
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::ContinueStmt) => Self::ContinueStmt(ContinueStmt(node)),
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::SwitchStmt) => Self::SwitchStmt(SwitchStmt(node)),
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::TryStmt) => Self::TryStmt(TryStmt(node)),
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::UnwindProtectStmt) => Self::UnwindProtectStmt(UnwindProtectStmt(node)),
            _ => Self::Expr(Expr::cast(node)?),
        };
        Some(result)
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Literal(Literal),
    ParenExpr(ParenExpr),
    UnaryExpr(UnaryExpr),
    VariableRef(VariableRef),
    MatrixExpr(MatrixExpr),
    CallExpr(CallExpr),
    PostfixExpr(PostfixExpr),
}

#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

#[derive(Debug)]
pub struct Literal(SyntaxNode);

#[derive(Debug)]
pub struct ParenExpr(SyntaxNode);

#[derive(Debug)]
pub struct UnaryExpr(SyntaxNode);

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;
    use parser::parse;

    fn get_root(input: &str) -> Root {
        let parse = parse(input);
        Root::cast(parse.syntax()).unwrap()
    }

    #[test]
    fn get_variable_defs() {
        let input = r#"a = 12;
b = a - 1
c = a * b"#;
        let root = get_root(input);
        let v: Vec<_> = root.get_variable_definitions().collect();
        let output = format!("{v:?}");
        let expected_output = expect![
            "[VariableDef(InfixExpr@0..8), VariableDef(InfixExpr@8..18), VariableDef(InfixExpr@18..27)]"
        ];

        expected_output.assert_eq(&output);
    }

    #[test]
    fn get_variable_refs() {
        let input = r#"a = 12;
a + 3
b = a - 1"#;
        let root = get_root(input);
        let v = root.get_variable_references();

        let output = format!("{v:?}");
        let expected_output = expect![[
            "[VariableRef(VariableRef@0..2), VariableRef(VariableRef@8..10), VariableRef(VariableRef@14..16), VariableRef(VariableRef@18..20)]"
        ]];

        expected_output.assert_eq(&output);
    }
}
