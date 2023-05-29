use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process::{self, exit},
};

mod interpreter;
mod lexer;
mod parser;
mod statements;
mod tokens;

use colored::Colorize;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use snafu::Whatever;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new(30);

    if args.len() > 2 {
        println!("Usage: <executable> [script].");
        process::exit(-1);
    } else if args.len() == 2 {
        run_file(&mut interpreter, args.get(1).unwrap().to_string());
    } else {
        run_prompt(&mut interpreter);
    }
}

fn run_file(interpreter: &mut Interpreter, fname: String) {
    let mut file = File::open(fname).expect("Unable to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read data from file.");
    match run(interpreter, data) {
        Ok(_) => pok(String::from("Execution completed sucessfully.")),
        Err(e) => {
            perror(e.to_string());
            exit(1)
        }
    }
}

fn run_prompt(interpreter: &mut Interpreter) {
    loop {
        print!("brainf*ck :> ");
        io::stdout().flush().expect("Unable to flush stdout.");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Unable to read line.");
        match run(interpreter, line) {
            Ok(_) => continue,
            Err(e) => perror(e.to_string()),
        }
    }
}

fn run(interpreter: &mut Interpreter, src: String) -> Result<(), Whatever> {
    let tokens = Lexer::new().scan(src);
    let statements = match Parser::new().parse(tokens) {
        Ok(s) => s,
        Err(e) => {
            return Err(e);
        }
    };
    return interpreter.interpret(statements);
}

fn pok(msg: String) {
    println!("\n[{}]: {}", "OK".green(), msg)
}

fn perror(msg: String) {
    println!("\n[{}]: {}", "ERR".red(), msg)
}
