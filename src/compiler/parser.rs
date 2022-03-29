use super::lexer::Token;
use anyhow::Result;

pub(crate) struct ParseResult {}

struct Ast {
    root: AstNode,
}

enum AstNode {
    Statement(Stmt),
    Expression(Expr),
}

enum Expr {
    /// `f(a, b)`
    FnCall(Box<(Expr, Vec<Expr>)>),
    Assign,
    Identifier,
}

enum Stmt {
    /// `f(a, b) { stmt }`
    FnDef,
}

pub(crate) fn parse(tokens: &Vec<Token>) -> Result<ParseResult> {
    todo!()
}
