
#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    // Single Characters
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two characters
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    Var,
    And, Or,
    If, Else,
    For, While,
    True, False,
    Nil,
    Class, Super, This, Fun,
    Return, Print,

    Eof,
}