use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Option<Literal>),
    Grouping(Option<Grouping>),
    Logical(Option<Logical>),
    Unary(Option<Unary>),
    Binary(Option<Binary>),
    Variable(Option<Variable>),
    Assign(Option<Assign>)
}

#[derive(Clone, Debug)]
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

    pub fn set_left(&mut self, left: Box<Expr>) {
        self.left = left;
    }

    pub fn set_right(&mut self, right: Box<Expr>) {
        self.right = right;
    }
}

#[derive(Clone, Debug)]
pub struct Logical {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>
}

impl Logical {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Logical {
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

    pub fn set_left(&mut self, left: Box<Expr>) {
        self.left = left;
    }

    pub fn set_right(&mut self, right: Box<Expr>) {
        self.right = right;
    }
}

#[derive(Clone, Debug)]
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

    pub fn set_expression(&mut self, expression: Box<Expr>) {
        self.expression = expression;
    }
}

#[derive(Clone, Debug)]
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

    pub fn set_expression(&mut self, expression: Box<Expr>) {
        self.expression = expression;
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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