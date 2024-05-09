use crate::token_type::TokenType;

#[derive(Debug)]
pub enum LiteralPossibleValues {
    StringValue(String),
    DoubleValue(f64),
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
    literal: Option<LiteralPossibleValues>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralPossibleValues>, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let type_ = match &self.token_type {
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Minus => "Minus",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Slash => "Slash",
            TokenType::Star => "Star",

            TokenType::Bang => "Bang",
            TokenType::BangEqual => "BangEqual",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Less => "Less",
            TokenType::LessEqual => "LessEqual",

            TokenType::Identifier => "Identifier",
            TokenType::String => "String",
            TokenType::Number => "Number",

            TokenType::Var => "Var",

            TokenType::And => "And",
            TokenType::Or => "Or",

            TokenType::If => "If",
            TokenType::Else => "Else",

            TokenType::For => "For",
            TokenType::While => "While",

            TokenType::True => "True",
            TokenType::False => "False",

            TokenType::Nil => "Nil",

            TokenType::Class => "Class",
            TokenType::Super => "Super",
            TokenType::This => "This",
            TokenType::Fun => "Fun",
            TokenType::Return => "Return",

            TokenType::Print => "Print",

            TokenType::Eof => "Eof",
        };

        format!("{} {} {:?}", type_, self.lexeme, self.literal)
    }
}