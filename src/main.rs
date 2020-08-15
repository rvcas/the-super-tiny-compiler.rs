use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
enum Token {
    Paren(char),
    Number(String),
    TString(String),
    Name(String),
}

fn tokenizer(input: &str) -> Vec<Token> {
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
            '"' => {
                let mut value = String::new();

                while let Some(sub) = chars.peek() {
                    match sub {
                        '"' => {
                            chars.next();
                            break;
                        }
                        _ => {
                            if let Some(next) = chars.next() {
                                value.push(next);
                            } else {
                                unreachable!()
                            }
                        }
                    }
                }

                tokens.push(Token::TString(value));
            }
            'a'..='z' => {
                let mut value = String::new();

                value.push(ch);

                while let Some(sub) = chars.peek() {
                    match sub {
                        'a'..='z' => {
                            if let Some(next) = chars.next() {
                                value.push(next);
                            } else {
                                unreachable!()
                            }
                        }
                        _ => break,
                    }
                }

                tokens.push(Token::Name(value))
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

                    let tokens = tokenizer(s.as_str());

                    println!("\u{001b}[37;1mTokens: \u{001b}[35;1m{:?}", tokens);

                    s.clear();
                }
            } else {
                let tokens = tokenizer(arg.as_str());

                println!("\u{001b}[37;1mTokens: \u{001b}[35;1m{:?}", tokens);
            }
        }
    }
}
