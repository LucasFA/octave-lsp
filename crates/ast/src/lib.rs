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
impl_typed_syntax_node!(VariableDef);
impl_typed_syntax_node!(Root);

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
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxConstruct::VariableRef.into())
            .unwrap()
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == TokenKind::Identifier.into())
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children().last().and_then(Expr::cast)
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
                SyntaxConstruct::Error => return None,
                SyntaxConstruct::Root => unreachable!(),
                SyntaxConstruct::VariableDef => unreachable!(),
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
                        TokenKind::Plus | TokenKind::Minus | TokenKind::Asterisk | TokenKind::Slash,
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
            .find(|token| token.kind() == TokenKind::Minus.into())
    }
}

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

impl Literal {
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
    #[must_use]
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }
}

#[derive(Debug)]
pub enum Stmt {
    VariableDef(VariableDef),
    Expr(Expr),
}

impl Stmt {
    #[must_use]
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::SyntaxConstruct(SyntaxConstruct::VariableDef) => {
                Self::VariableDef(VariableDef(node))
            }
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
b = a - 1"
c = a * b"#;
        let root = get_root(input);
        let v: Vec<_> = root.get_variable_definitions().collect();
        let output = format!("{v:?}");
        let expected_output = expect!["[VariableDef(VariableDef@0..8), VariableDef(VariableDef@8..17), VariableDef(VariableDef@19..28)]"];

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
        let expected_output = expect!["[VariableRef(VariableRef@0..2), VariableRef(VariableRef@8..10), VariableRef(VariableRef@14..16), VariableRef(VariableRef@18..20)]"];

        expected_output.assert_eq(&output);
    }
}
