use crate::token::{LiteralPossibleValues, Token};
use crate::token_type::TokenType;
use crate::error_hadling::*;
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let token: Token = Token::new(TokenType::Eof, String::new(), None, self.line);

        self.tokens.push(token);

        &self.tokens
    }

    fn scan_token(&mut self) {
        let character = self.advance();

        match character {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {if self.match_next('=') {self.add_token(TokenType::BangEqual);} else {self.add_token(TokenType::Bang);}},
            '=' => {if self.match_next('=') {self.add_token(TokenType::EqualEqual);} else {self.add_token(TokenType::Equal);}},
            '<' => {if self.match_next('=') {self.add_token(TokenType::LessEqual);} else {self.add_token(TokenType::Less);}},
            '>' => {if self.match_next('=') {self.add_token(TokenType::GreaterEqual);} else {self.add_token(TokenType::Greater);}},

            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                else if self.match_next('*') {
                    while (self.peek() != '*' || self.peek_next() != '/') && !self.is_at_end() {
                        self.advance();
                    }
                    self.advance();
                    self.advance();
                }
                else {
                    self.add_token(TokenType::Slash);
                }
            },

            '"' => self.string(),

            ' ' => {},
            '\t' => {},
            '\r' => {},

            '\n' => self.line += 1,

            _ => {
                if self.is_digit(character) {
                    self.number();
                }
                else if self.is_alpha(character) {
                    self.identifier();
                }
                else {
                    error(self.line, format!("Unexpected character: {}", character).as_str());
                }
            },
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let source_string: String = self.source.clone();
        let sub_string: String = source_string.get((self.start as usize)..(self.current as usize)).unwrap_or_default().to_string();

        let value = self.keywords.get(&sub_string).cloned();

        if let Some(val) = value {
            self.add_token(val);
        }
        else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let source_string: String = self.source.clone();
        let number_string: String = source_string.get((self.start as usize)..(self.current as usize)).unwrap_or_default().to_string();

        let number_double: f64 = number_string.parse().unwrap_or(0.0);

        self.add_token_in_list(TokenType::Number, Some(LiteralPossibleValues::DoubleValue(number_double)));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
        }

        self.advance(); // for the last "

        let source_string: String = self.source.clone();
        let value: String = source_string.get((self.start as usize)+1..(self.current as usize)-1).unwrap_or_default().to_string();

        self.add_token_in_list(TokenType::String, Some(LiteralPossibleValues::StringValue(value)));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let character = self.source.chars().nth(self.current as usize);

        if let Some(value) = character {
            if value != expected {
                return false;
            }
            else {
                self.current += 1;
                return true;
            }
        } else {
            return false;
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let character = self.source.chars().nth(self.current as usize);

        if let Some(value) = character {
            return value;
        } else {
            return '\0';
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as i32 {
            return '\0';
        }
        else {
            let character = self.source.chars().nth((self.current + 1) as usize);

            if let Some(value) = character {
                return value;
            }
            else {
                return '\0';
            }
        }
    }

    fn is_alpha(&self, character: char) -> bool {
        let a_lower: char = 'a';
        let z_lower: char = 'z';
        let a_upper: char = 'A';
        let z_upper: char = 'Z';
        let underscore: char = '_';

        ((character as i32) >= (a_lower as i32) && (character as i32) <= (z_lower as i32)) || ((character as i32) >= (a_upper as i32) && (character as i32) <= (z_upper as i32)) || character == underscore
    }

    fn is_alpha_numeric(&self, character: char) -> bool {
        self.is_alpha(character) || self.is_digit(character)
    }

    fn is_digit(&self, character: char) -> bool {
        let zero: char = '0';
        let nine: char = '9';

        (character as i32) >= (zero as i32) && (character as i32) <= (nine as i32)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    fn advance(&mut self) -> char {
        let character: Option<char> = self.source.chars().nth(self.current as usize);
        self.current += 1;

        match character {
            Some(a) => a,
            None => '\0'
        }
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_token_in_list(token, None);
    }

    fn add_token_in_list(&mut self, token_type: TokenType, literal: Option<LiteralPossibleValues>) {
        let source_string: String = self.source.clone();
        let sub_string: String = source_string.get(self.start as usize..self.current as usize).unwrap_or_default().to_string();

        let token: Token = Token::new(token_type, sub_string, literal, self.line);
        self.tokens.push(token);
    }


}