use crate::token::Token;

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
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
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
    pub fn new(operator: Token, expression: Box<Expr>) -> Self {
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
    pub fn new(operator: Token) -> Self {
        Operator {
            operator
        }
    }
}

#[derive(Clone)]
pub struct Grouping {
    expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Grouping {
            expression,
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
    pub fn new(value: Token) -> Self {
        Literal {
            value
        }
    }

    pub fn getValue(&self) -> Token {
        self.value.clone()
    }
}