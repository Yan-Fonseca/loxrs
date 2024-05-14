use std::vec;

use crate::{expr::*, token_type::TokenType};

pub struct AstPrinter;

impl AstPrinter {
    pub fn printBinaryExpr(&self, expr: Binary) -> String {
        let expressions = vec!(*expr.getLeft(), *expr.getRight());
        self.parenthesize(expr.getOperator().getLexeme(), &expressions)
    }

    pub fn printGroupingExpr(&self, expr: Grouping) -> String {
        let expressions = vec!(*expr.getExpression());
        self.parenthesize("group".to_string(), &expressions)
    }

    pub fn printLiteralExpr(&self, expr: Literal) -> String {
        let value = expr.getValue();
        match value.getTokenType() {
            TokenType::Nil => "nil".to_string(),
            _ => value.getLexeme()
        }
    }

    pub fn printUnaryExpr(&self, expr: Unary) -> String {
        let expression = vec!(*expr.getExpression());
        self.parenthesize(expr.getOperator().getLexeme(), &expression)
    }

    fn getNewPrint(&self, expr: Expr) -> String {
        match expr {
            Expr::Literal(value) => {
                if let Some(val) = value {
                    return self.printLiteralExpr(val);
                }
                else {
                    return "[ERROR]".to_string();
                }
            },
            Expr::Binary(value) => {
                if let Some(val) = value {
                    return self.printBinaryExpr(val);
                }
                else {
                    return "[ERROR]".to_string();
                }
            },
            Expr::Grouping(value) => {
                if let Some(val) = value {
                    return self.printGroupingExpr(val);
                }
                else {
                    return "[ERROR]".to_string();
                }
            },
            Expr::Unary(value) => {
                if let Some(val) = value {
                    return self.printUnaryExpr(val);
                }
                else {
                    return "[ERROR]".to_string();
                }
            },
            _ => return "Print Operator is not defined, update your code, please".to_string()
        }
    }

    fn parenthesize(&self, name: String, expr: &[Expr]) -> String {
        let mut builder: String = String::new();

        builder.push('(');
        builder.push_str(name.as_str());

        for expression in expr {
            builder.push(' ');
            let expr = expression.clone();

            builder.push_str(self.getNewPrint(expr).as_str());
        }
        builder.push(')');

        builder
    }
}