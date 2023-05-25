use std::env::args;
use std::io::{self, stdout, BufRead, Write};

mod error;
mod scanner;
mod tokens;

use error::*;
use scanner::*;

pub fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file."),
        _ => {
            println!("Incorrect Usage. Usage: sun [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;

    if run(buf).is_err() {
        std::process::exit(65);
    }

    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");

    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }

            let _ = run(line);
        } else {
            break;
        }

        print!("> ");
        let _ = stdout().flush();
    }
}

fn run(source: String) -> Result<(), SunError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
