use std::env;
use std::io::{stdin, stdout, Write};

mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => println!("Error: expected atleast one argument"),
        Some(arg) => {
            if arg == "repl" {
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

                    let tokens = tokenizer::from_str(s.as_str());

                    println!("\u{001b}[37;1mTokens: \u{001b}[35;1m{:?}", tokens);

                    s.clear();
                }
            } else {
                let tokens = tokenizer::from_str(arg.as_str());

                println!("\u{001b}[37;1mTokens: \u{001b}[35;1m{:?}", tokens);
            }
        }
    }
}
