use std::{
    env, fs,
    io::{self, Write},
};

mod error;
mod token;
mod scanner;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.iter().count() > 3 {
        println!("usage: rlox [script]");
        std::process::exit(64);
    } else if args.iter().count() == 2 {
        println!("usage: rlox [file]");
        run_file(args[1].clone());
    } else {
        run_prompt();
        println!("usage: rlox [prompt]");
    }
}

fn run_file(path: String) {
    println!("runFile: {}", path);
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    println!("contents: {}", contents);
}

fn run_prompt() {
    let mut input = String::new();
    loop {
        println!("> ");

        io::stdout().flush().unwrap();

        input.clear();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {break;}

        run(&mut input);
    }
}

fn run(input: &mut String) {
    println!("input: {}", input);
}
