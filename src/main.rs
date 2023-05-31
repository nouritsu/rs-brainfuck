use std::{
    fs::File,
    io::{self, Read, Write},
    process::exit,
    time::Instant,
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
#[command(author = "Aneesh B.", about, long_about=None)]
#[command(version = "1.0.0")]
#[command(about = "A fast, memory safe and powerful brainfuck interpreter.")]
struct Args {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long, default_value_t = 30000)]
    arrlen: usize,

    #[arg(short, long, default_value_t = false)]
    time: bool,
}

fn main() {
    let args = Args::parse();
    let mut interpreter = Interpreter::new(args.arrlen);
    let now = Instant::now();

    if let Some(f) = args.file {
        run_file(&mut interpreter, f, args.time);
    } else {
        run_prompt(&mut interpreter, args.time)
    }

    if args.time {
        pinfo(format!(
            "Total execution time: {:.3}s.",
            nano_to_sec(now.elapsed().as_nanos())
        ))
    }
}

fn run_file(interpreter: &mut Interpreter, fname: String, show_time: bool) {
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
    match run(interpreter, data, show_time) {
        Ok(_) => pok(String::from("Execution completed sucessfully.")),
        Err(e) => {
            perror(e.to_string());
            exit(-1)
        }
    }
}

fn run_prompt(interpreter: &mut Interpreter, show_time: bool) {
    loop {
        print!("brainf*ck :> ");
        io::stdout().flush().expect("Unable to flush stdout.");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Unable to read line.");
        // line.push('^');
        match run(interpreter, line, show_time) {
            Ok(_) => continue,
            Err(e) => perror(format!("\n{}", e.to_string())),
        }
    }
}

fn run(interpreter: &mut Interpreter, src: String, show_time: bool) -> Result<(), Whatever> {
    let now = Instant::now();
    let tokens = Lexer::new().scan(src);
    let lex_time = nano_to_sec(now.elapsed().as_nanos());

    let now = Instant::now();
    let instructions = Parser::new().parse(tokens)?;
    let parse_time = nano_to_sec(now.elapsed().as_nanos());

    let now = Instant::now();
    interpreter.interpret(&instructions)?;
    let interpret_time = nano_to_sec(now.elapsed().as_nanos());
    println!();

    if show_time {
        pinfo(format!("Lexed string in: {:.3}s", lex_time));
        pinfo(format!("Parsed tokens in: {:.3}s", parse_time));
        pinfo(format!("Executed instructions in: {:.3}s", interpret_time));
    }

    Ok(())
}

fn pok(msg: String) {
    println!("[{}]: {}", "OK".green(), msg)
}

fn pinfo(msg: String) {
    println!("[{}]: {}", "INFO".blue(), msg)
}

fn perror(msg: String) {
    println!("[{}]: {}", "ERR".red(), msg)
}

fn nano_to_sec(ns: u128) -> f64 {
    ns as f64 / 1_000_000_000_f64
}
