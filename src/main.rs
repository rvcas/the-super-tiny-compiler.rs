use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
enum Token {
    Paren(char),
    Number(String),
}

fn tokenizer(input: &str) -> Vec<Token> {
    let _current = 0;

    let mut tokens: Vec<Token> = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' => {}
            '(' | ')' => tokens.push(Token::Paren(ch)),
            '0'..='9' => {
                let mut value = String::new();

                value.push(ch);

                while let Some(sub) = chars.peek() {
                    match sub {
                        '0'..='9' => {
                            if let Some(next) = chars.next() {
                                value.push(next);
                            } else {
                                unreachable!()
                            }
                        }
                        _ => break,
                    }
                }

                tokens.push(Token::Number(value));
            }

            _ => {
                println!("I dont know what this character is: {}", ch);

                break;
            }
        }
    }

    return tokens;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => println!("Error: expected atleast one argument"),
        Some(arg) => {
            let mut s = String::new();
            println!("{}", arg);

            if arg == "repl" {
                loop {
                    print!("tiny> ");

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

                    let tokens = tokenizer(s.as_str());

                    println!("Tokens: {:?}", tokens);

                    s.clear();
                }
            } else {
                let tokens = tokenizer(arg.as_str());

                println!("Tokens: {:?}", tokens);
            }
        }
    }
}
