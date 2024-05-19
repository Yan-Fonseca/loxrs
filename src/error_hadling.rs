use crate::{token::Token, token_type::TokenType};

pub static mut HAD_ERROR: bool = false;
pub static mut HAD_RUNTIME_ERROR: bool = false;

pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

pub fn runtime_error(token: Token, message: String) {
    println!("{} \n[line {}]", message, token.get_line());
    unsafe {
        HAD_RUNTIME_ERROR = true;
    }
}

pub fn report(line: i32, where_err: &str, message: &str) {
    println!("[line {}] Error {} : {}", line, where_err, message);
    unsafe {
        HAD_ERROR = true;
    }
}

pub fn parser_error(token: Token, message: String) {
    if token.get_token_type() == TokenType::Eof {
        report(token.get_line(), " at end", message.as_str());
    }
    else {
        report(token.get_line(), (" at '".to_string()+ token.get_lexeme().as_str() + "'").as_str(), message.as_str());
    }
}