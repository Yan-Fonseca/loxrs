use std::env;
use std::io;
use std::io::Write;
use std::fs;
use std::str;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(0);
    }
    else if args.len() == 2{
        println!("Argumento: {}", args[1]);
        let _ = run_file(&args[1]);
    }
    else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let bytes = fs::read(path)?;

    let content = str::from_utf8(&bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    run(content);

    Ok(())
}


fn run_prompt() {
    let stdin = io::stdin();
    let mut buffer = String::new();

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

                run(input);
            },
            Err(error) => {
                eprintln!("Erro ao ler a entrada: {}", error);
                break;
            }
        }
    }
}

fn run(input: &str) {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    for token in tokens {
        println!("{:?}", token);
    }
}
