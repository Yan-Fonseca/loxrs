use crate::token::Token;

#[derive(Clone)]
pub enum Expr {
    Literal(Option<Literal>),
    Grouping(Option<Grouping>),
    Unary(Option<Unary>),
    Binary(Option<Binary>),
    Variable(Option<Variable>),
    Assign(Option<Assign>)
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

    pub fn get_left(&self) -> Box<Expr> {
        self.left.clone()
    }

    pub fn get_operator(&self) -> Token {
        self.operator.clone()
    }

    pub fn get_right(&self) -> Box<Expr> {
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

    pub fn get_operator(&self) -> Token {
        self.operator.clone()
    }

    pub fn get_expression(&self) -> Box<Expr> {
        self.expression.clone()
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

    pub fn get_expression(&self) -> Box<Expr> {
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

    pub fn get_value(&self) -> Token {
        self.value.clone()
    }
}

#[derive(Clone)]
pub struct Variable {
    value: Token
}

impl Variable {
    pub fn new(value: Token) -> Self {
        Variable {
            value
        }
    }

    pub fn get_value(&self) -> Token {
        self.value.clone()
    }
}

#[derive(Clone)]
pub struct Assign {
    value: Token,
    expression: Box<Expr>
}

impl Assign {
    pub fn new(value: Token, expression: Box<Expr>) -> Self {
        Assign {
            value,
            expression
        }
    }

    pub fn get_value(&self) -> Token {
        self.value.clone()
    }

    pub fn get_expression(&self) -> Box<Expr> {
        self.expression.clone()
    }
}