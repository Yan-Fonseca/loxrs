pub static mut HAD_ERROR: bool = false;

pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

pub fn report(line: i32, where_err: &str, message: &str) {
    println!("[line {}] Error {} : {}", line, where_err, message);
    unsafe {
        HAD_ERROR = true;
    }
}