use std::vec;

use crate::{expr::*, token_type::TokenType};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }
 
    pub fn print(&self, expr: Expr) {
        println!("{}", self.get_new_print(expr));
    }

    pub fn print_binary_expr(&self, expr: Binary) -> String {
        let expressions = vec!(*expr.get_left(), *expr.get_right());
        self.parenthesize(expr.get_operator().get_lexeme(), &expressions)
    }

    pub fn print_grouping_expr(&self, expr: Grouping) -> String {
        let expressions = vec!(*expr.get_expression());
        self.parenthesize("group".to_string(), &expressions)
    }

    pub fn print_literal_expr(&self, expr: Literal) -> String {
        let value = expr.get_value();
        match value.get_token_type() {
            TokenType::Nil => "nil".to_string(),
            _ => value.get_lexeme()
        }
    }

    pub fn print_unary_expr(&self, expr: Unary) -> String {
        let expression = vec!(*expr.get_expression());
        self.parenthesize(expr.get_operator().get_lexeme(), &expression)
    }

    pub fn print_assign_expr(&self, expr: Assign) -> String {
        let expression = vec!(*expr.get_expression());
        self.parenthesize("(assign)= ".to_string() + &expr.get_value().get_lexeme(), &expression)
    }

    pub fn print_variable_expr(&self, expr: Variable) -> String {
        let expression = expr.get_value();
        return String::from(expression.get_lexeme());
    }

    fn get_new_print(&self, expr: Expr) -> String {
        match expr {
            Expr::Literal(value) => {
                if let Some(val) = value {
                    return self.print_literal_expr(val);
                }
                else {
                    return "[ERROR] Literal".to_string();
                }
            },
            Expr::Binary(value) => {
                if let Some(val) = value {
                    return self.print_binary_expr(val);
                }
                else {
                    return "[ERROR] Binary".to_string();
                }
            },
            Expr::Grouping(value) => {
                if let Some(val) = value {
                    return self.print_grouping_expr(val);
                }
                else {
                    return "[ERROR] Grouping".to_string();
                }
            },
            Expr::Unary(value) => {
                if let Some(val) = value {
                    return self.print_unary_expr(val);
                }
                else {
                    return "[ERROR] Unary".to_string();
                }
            },
            Expr::Variable(value) => {
                if let Some(val) = value {
                    return self.print_variable_expr(val);
                }
                else {
                    return "[ERROR] Variable".to_string();
                }
            },
            Expr::Assign(value) => {
                if let Some(val) = value {
                    return self.print_assign_expr(val);
                }
                else {
                    return "[ERROR] Assign".to_string();
                }
            },
            _ => return "[ERROR]".to_string()
        }
    }

    fn parenthesize(&self, name: String, expr: &[Expr]) -> String {
        let mut builder: String = String::new();

        builder.push('(');
        builder.push_str(name.as_str());

        for expression in expr {
            builder.push(' ');
            let expr = expression.clone();

            builder.push_str(self.get_new_print(expr).as_str());
        }
        builder.push(')');

        builder
    }
}