use crate::{token::Token, token_type::TokenType};

pub static mut HAD_ERROR: bool = false;

pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

pub fn report(line: i32, where_err: &str, message: &str) {
    println!("[line {}] Error {} : {}", line, where_err, message);
    // unsafe {
    //     HAD_ERROR = true;
    // }
}

pub fn parser_error(token: Token, message: String) {
    if token.getTokenType() == TokenType::Eof {
        report(token.getLine(), " at end", message.as_str());
    }
    else {
        report(token.getLine(), (" at '".to_string()+ token.getLexeme().as_str() + "'").as_str(), message.as_str());
    }
}