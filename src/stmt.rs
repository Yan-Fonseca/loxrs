use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Expr(Expr),
    Block(Vec<Stmt>),
    Print(Expr),
    Var(Token, Option<Expr>)
}
