use jlang::*;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    // Lexical analysis
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            process::exit(1);
        }
    };

    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parser error: {:?}", err);
            process::exit(1);
        }
    };

    println!("\nAST:");
    println!("{:#?}", ast);
}
