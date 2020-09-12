use std::env;
use std::io::{stdin, stdout, Write};

mod parser;
mod tokenizer;
mod transformer;

fn compile(source: &str) {
    let tokens = tokenizer::from_str(source);

    println!("\u{001b}[37;1mTokens: \u{001b}[35;1m{:?}\n", tokens);

    let ast = parser::ast_from_tokens(tokens);

    println!("\u{001b}[37;1mAST: \u{001b}[35;1m{:?}\n", ast);

    let c_ast = transformer::c_ast_from_program(ast);

    println!("\u{001b}[37;1mC AST: \u{001b}[35;1m{:?}", c_ast);
}

fn start_repl() {
    let mut s = String::new();

    loop {
        print!("\u{001b}[31;1mtiny> \u{001b}[36m");

        let _ = stdout().flush();

        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }

        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        if s == ":q" || s == ":quit" {
            s.clear();
            break;
        }

        compile(s.as_str());

        s.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => println!("Error: expected atleast one argument"),
        Some(arg) => {
            if arg == "repl" {
                start_repl()
            } else {
                compile(arg.as_str())
            }
        }
    }
}
