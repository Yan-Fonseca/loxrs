use crate::token::{Token};

#[derive(Clone)]
pub enum Expr {
    Literal(Option<Literal>),
    Grouping(Option<Grouping>),
    Unary(Option<Unary>),
    Binary(Option<Binary>),
    Operator(Option<Operator>)
}

#[derive(Clone)]
pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>
}

impl Binary {
    fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Binary {
            left,
            operator,
            right
        }
    }

    pub fn getLeft(&self) -> Box<Expr> {
        self.left.clone()
    }

    pub fn getOperator(&self) -> Token {
        self.operator.clone()
    }

    pub fn getRight(&self) -> Box<Expr> {
        self.right.clone()
    }
}

#[derive(Clone)]
pub struct Unary {
    operator: Token,
    expression: Box<Expr>,
}

impl Unary {
    fn new(operator: Token, expression: Box<Expr>) -> Self {
        Unary {
            operator,
            expression
        }
    }

    pub fn getOperator(&self) -> Token {
        self.operator.clone()
    }

    pub fn getExpression(&self) -> Box<Expr> {
        self.expression.clone()
    }
}

#[derive(Clone)]
pub struct Operator {
    operator: Token,
}

impl Operator {
    fn new(operator: Token) -> Self {
        Operator {
            operator
        }
    }
}

#[derive(Clone)]
pub struct Grouping {
    left: Token,
    expression: Box<Expr>,
    right: Token,
}

impl Grouping {
    fn new(left: Token, expression: Box<Expr>, right: Token) -> Self {
        Grouping {
            left,
            expression,
            right
        }
    }

    pub fn getExpression(&self) -> Box<Expr> {
        self.expression.clone()
    }
}

#[derive(Clone)]
pub struct Literal {
    value: Token,
}

impl Literal {
    fn new(value: Token) -> Self {
        Literal {
            value
        }
    }

    pub fn getValue(&self) -> Token {
        self.value.clone()
    }
}