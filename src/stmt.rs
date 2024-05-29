use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Expr(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Block(Vec<Stmt>),
    Print(Expr),
    Var(Token, Option<Expr>)
}
