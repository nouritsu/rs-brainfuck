use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process,
};

mod interpreter;
mod lexer;
mod tokens;

use interpreter::Interpreter;
use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let interpreter = Interpreter::new(300000);

    if args.len() > 2 {
        println!("Usage: <executable> [script].");
        process::exit(-1);
    } else if args.len() == 2 {
        run_file(args.get(1).unwrap().to_string());
    } else {
        run_prompt();
    }
}

fn run_file(fname: String) {
    let mut file = File::open(fname).expect("Unable to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read data from file.");
    run(data);
}

fn run_prompt() {
    loop {
        print!("brainf*ck :> ");
        io::stdout().flush().expect("Unable to flush stdout.");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Unable to read line.");
        run(line);
    }
}

fn run(src: String) {
    let tokens = Lexer::new(src).scan();
    println!("{:?}", tokens);
}
