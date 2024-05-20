use crate::token::Token;
use crate::token_type::TokenType;
use crate::expr::*;
use crate::stmt::*;
use crate::error_hadling::parser_error;

pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current = 0;
        Parser {
            tokens,
            current
        }
    }

    pub fn parser(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(value) => {
                    if let Some(val) = value {
                        statements.push(val);
                    }
                },
                Err(e) => println!("{}", e),
            }
        }

        statements
    }

    fn declaration(&mut self) -> Result<Option<Stmt>, String> {
        let result = self.declaration_aux();

        match result {
            Ok(value) => Ok(value),
            Err(_) => {
                self.syncronize();
                return Ok(None);
            },
        }
    }

    fn declaration_aux(&mut self) -> Result<Option<Stmt>, String> {
        if self.match_signal(&vec![TokenType::Var]) {
            return self.var_declaration();
        }
        return self.statement();
    }

    fn var_declaration(&mut self) -> Result<Option<Stmt>, String> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string())?;

        let mut initializer: Option<Expr> = None;

        if self.match_signal(&vec![TokenType::Equal]) {
            let aux = self.expression()?;
            initializer = Some(aux);
        }

        let _ = self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.".to_string());

        Ok(Some(Stmt::Var(name, initializer)))
    }

    fn statement(&mut self) -> Result<Option<Stmt>, String> {
        let _types = &vec![TokenType::Print];
        if self.match_signal(_types) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Option<Stmt>, String> {
        let value = self.expression();
        match value {
            Ok(val) => {
                let _ = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
                return Ok(Some(Stmt::Print(val)));
            },
            Err(e) => Err(e),
        }
    }

    fn expression_statement(&mut self) -> Result<Option<Stmt>, String> {
        let value = self.expression();
        match value {
            Ok(expr) => {
                let _ = self.consume(TokenType::Semicolon, "Expect ';' after expression.".to_string());
                return Ok(Some(Stmt::Expr(expr)));
            },
            Err(e) => Err(e),
        }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        let types = &vec![TokenType::BangEqual, TokenType::EqualEqual];

        while self.match_signal(types) {
            let operator = self.previous();
            let right = self.comparison()?;

            let left_pointer = Box::new(expr);
            let right_pointer = Box::new(right);

            expr = Expr::Binary(Some(Binary::new(left_pointer, operator, right_pointer)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        let types = &vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual];

        while self.match_signal(types) {
            let operator = self.previous();
            let right = self.term()?;

            let left_pointer = Box::new(expr);
            let right_pointer = Box::new(right);

            expr = Expr::Binary(Some(Binary::new(left_pointer, operator, right_pointer)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        let types = &vec![TokenType::Plus, TokenType::Minus];

        while self.match_signal(types) {
            let operator = self.previous();
            let right = self.factor()?;

            let left_pointer = Box::new(expr);
            let right_pointer = Box::new(right);

            expr = Expr::Binary(Some(Binary::new(left_pointer, operator, right_pointer)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        let types = &vec![TokenType::Slash, TokenType::Star];

        while self.match_signal(types) {
            let operator = self.previous();
            let right = self.unary()?;

            let left_pointer = Box::new(expr);
            let right_pointer = Box::new(right);

            expr = Expr::Binary(Some(Binary::new(left_pointer, operator, right_pointer)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        let types = &vec![TokenType::Bang, TokenType::Minus];

        while self.match_signal(types) {
            let operator = self.previous();
            let right = self.unary()?;

            let right_pointer = Box::new(right);

            return Ok(Expr::Unary(Some(Unary::new(operator, right_pointer))));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, String> {
        // False case
        if self.match_signal(&vec![TokenType::False]) {
            return Ok(Expr::Literal(Some(Literal::new(self.previous()))));
        }

        // True case
        if self.match_signal(&vec![TokenType::True]) {
            return Ok(Expr::Literal(Some(Literal::new(self.previous()))));
        }

        // Nil case
        if self.match_signal(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Some(Literal::new(self.previous()))));
        }

        // String or Number case
        if self.match_signal(&vec![TokenType::String, TokenType::Number]) {
            return Ok(Expr::Literal(Some(Literal::new(self.previous()))));
        }

        // Grouping case
        if self.match_signal(&vec![TokenType::LeftParen]) {
            let expression = self.expression()?;
            let _ = self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());

            let expression = Box::new(expression);

            return Ok(Expr::Grouping(Some(Grouping::new(expression))));
        }

        if self.match_signal(&vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(Some(Variable::new(self.previous()))));
        }

        Err(self.error(self.peek(), "Expect expression.".to_string()))

    }

    fn match_signal(&mut self, types: &Vec<TokenType>) -> bool {
        for _type in types {
            if self.check(_type.clone()) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn consume(&mut self, _type: TokenType, message: String) -> Result<Token, String> {
        if self.check(_type) {
            return Ok(self.advance());
        }

        return Err(self.error(self.peek(), message));
    }

    fn check(&self, _type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        else {
            return self.peek().get_token_type() == _type;
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().get_token_type() == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current as usize).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get((self.current as usize) - 1).unwrap().clone()
    }

    fn error(&self, token: Token, message: String) -> String {
        parser_error(token, message);

        "Parser ERROR".to_string()
    }

    fn syncronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().get_token_type() == TokenType::Semicolon {
                return;
            }

            match self.peek().get_token_type() {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}