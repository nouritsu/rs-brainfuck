use std::{
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

mod instructions;
mod interpreter;
mod lexer;
mod parser;
mod tokens;

use colored::Colorize;

use clap::Parser as ArgParser;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use snafu::Whatever;

#[derive(ArgParser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long, default_value_t = 30000)]
    arrlen: usize,
}

fn main() {
    let args = Args::parse();
    let mut interpreter = Interpreter::new(args.arrlen);

    if let Some(f) = args.file {
        run_file(&mut interpreter, f);
    } else {
        run_prompt(&mut interpreter)
    }
}

fn run_file(interpreter: &mut Interpreter, fname: String) {
    let mut file = match File::open(fname) {
        Ok(f) => f,
        Err(e) => {
            perror(e.to_string());
            exit(2)
        }
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => {}
        Err(e) => {
            perror(e.to_string());
            exit(1)
        }
    }
    match run(interpreter, data) {
        Ok(_) => pok(String::from("Execution completed sucessfully.")),
        Err(e) => {
            perror(e.to_string());
            exit(-1)
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
        // line.push('^');
        match run(interpreter, line) {
            Ok(_) => continue,
            Err(e) => perror(e.to_string()),
        }
    }
}

fn run(interpreter: &mut Interpreter, src: String) -> Result<(), Whatever> {
    let tokens = Lexer::new().scan(src);
    let statements = Parser::new().parse(tokens)?;
    return interpreter.interpret(&statements);
}

fn pok(msg: String) {
    println!("\n[{}]: {}", "OK".green(), msg)
}

fn perror(msg: String) {
    println!("\n[{}]: {}", "ERR".red(), msg)
}
