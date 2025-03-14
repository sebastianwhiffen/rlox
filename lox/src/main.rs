use std::{
    env, fs,
    io::{self, Write},
};

use ast::{
    expr::expr::Expr,
    printer::printer::Printer,
    token::{token::TokenType, Token},
};

mod ast;
mod error;
mod parser;
mod scanner;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.iter().count() > 3 {
    //     println!("usage: rlox [script]");
    //     std::process::exit(64);
    // } else if args.iter().count() == 2 {
    //     println!("usage: rlox [file]");
    //     run_file(args[1].clone());
    // } else {
    //     run_prompt();
    //     println!("usage: rlox [prompt]");
    // }

    let expr: Expr = Expr::Binary(
        Box::new(Expr::Unary(
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_owned(),
                line: 1,
                ..Token::default()
            },
            Box::new(Expr::Literal(Token {
                token_type: TokenType::Number,
                lexeme: "123".to_owned(),
                line: 1,
                ..Token::default()
            })),
        )),
        Token {
            token_type: TokenType::Star,
            lexeme: "*".to_owned(),
            line: 1,
            ..Token::default()
        },
        Box::new(Expr::Grouping(Box::new(Expr::Literal(Token {
            token_type: TokenType::Number,
            lexeme: "123".to_owned(),
            line: 1,
            ..Token::default()
        })))),
    );

    print!("{}", Printer {}.print(expr))
}

fn run_file(path: String) {
    println!("runFile: {}", path);
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    println!("contents: {}", contents);
}

fn run_prompt() {
    let mut input = String::new();
    let mut had_error = false;
    loop {
        println!("> ");

        io::stdout().flush().unwrap();

        input.clear();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            break;
        }

        run(&mut input);
    }
}

fn run(input: &mut String) {
    let mut scanner = scanner::Scanner {
        source: input.clone(),
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
    };

    if let Err(errors) = scanner.scan_tokens() {
        for error in errors.iter() {
            println!("{}", error);
        }
    };

    for token in scanner.tokens.iter() {
        println!("{}", token.to_string());
    }
}
