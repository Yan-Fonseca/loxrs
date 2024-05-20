pub mod token;
pub mod token_type;
pub mod scanner;
pub mod error_hadling;
pub mod expr;
pub mod ast_printer;
pub mod parser;
pub mod interpreter;
pub mod stmt;
pub mod environment;

use std::env;
use std::io;
use std::io::Write;
use std::fs;
use std::str;

use error_hadling::HAD_ERROR;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(0);
    }
    else if args.len() == 2{
        let _ = run_file(&args[1]);
    }
    else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let bytes = fs::read(path)?;

    let content = str::from_utf8(&bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut interpreter: Interpreter = Interpreter::new();

    run(content, &mut interpreter);

    unsafe {
        if error_hadling::HAD_ERROR || error_hadling::HAD_RUNTIME_ERROR {
            std::process::exit(0);
        }
    }

    Ok(())
}


fn run_prompt() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut interpreter: Interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        buffer.clear();

        match stdin.read_line(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    println!("[Exiting]");
                    break;
                }

                let input = buffer.trim();
                if input.is_empty() {
                    continue;
                }

                run(input, &mut interpreter);

                unsafe {
                    error_hadling::HAD_ERROR = false;
                }
            },
            Err(error) => {
                eprintln!("Erro ao ler a entrada: {}", error);
                break;
            }
        }
    }
}

fn run(input: &str, interpreter: &mut Interpreter) {
    let mut scanner = Scanner::new(input.to_string());

    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.to_vec());

    let statements = parser.parser();

    if unsafe { HAD_ERROR } {
        return;
    }

    interpreter.interpret(statements);
}