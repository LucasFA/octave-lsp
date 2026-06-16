#![warn(clippy::pedantic)]

mod database;
pub use database::Database;

use la_arena::Idx;
use smol_str::SmolStr;

type ExprIdx = Idx<Expr>;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VariableDef { name: SmolStr, value: Expr },
    FnDef { body: Vec<Stmt> },
    If { condition: ExprIdx, body: Vec<Stmt> },
    ForLoop { body: Vec<Stmt> },
    WhileLoop { condition: ExprIdx, body: Vec<Stmt> },
    Switch { condition: ExprIdx, body: Vec<Stmt> },
    Try { body: Vec<Stmt>, catch: Vec<Stmt> },
    UnwindProtect { body: Vec<Stmt>, cleanup: Vec<Stmt> },
    Break,
    Continue,
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: ExprIdx,
        rhs: ExprIdx,
    },
    Literal {
        n: Option<u64>,
    },
    Unary {
        op: UnaryOp,
        expr: ExprIdx,
    },
    VariableRef {
        var: SmolStr,
    },
    Call {
        func: ExprIdx,
        args: Vec<ExprIdx>,
    },
    Matrix {
        elements: Vec<ExprIdx>,
    },
    Range {
        lhs: ExprIdx,
        rhs: ExprIdx,
    },
    Transpose {
        op: TransposeOp,
        expr: ExprIdx,
    },
    String {
        value: SmolStr,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    ElmtMult,
    ElmtDiv,
    LeftDiv,
    ElmtLeftDiv,
    Pow,
    ElmtPow,
    Colon,
    EqualsEquals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    And,
    Or,
    Assign,
    PlusEquals,
    MinusEquals,
    AsteriskEquals,
    SlashEquals,
    ElmtMultEquals,
    ElmtDivEquals,
    ElmtPowEquals,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
    Plus,
    Not,
    Tilde,
}

#[derive(Debug, PartialEq)]
pub enum TransposeOp {
    Normal,
    Elmt,
}

#[must_use]
pub fn lower(ast: &ast::Root) -> (Database, Vec<Stmt>) {
    let mut db = Database::default();
    let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();

    (db, stmts)
}
