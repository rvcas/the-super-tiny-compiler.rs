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
                                break;
                            }
                        }
                        _ => break,
                    }
                }

                tokens.push(Token::Number(value));
            }

            _ => panic!("I dont know what this character is: {}", ch),
        }
    }

    return tokens;
}

fn main() {
    let tokens = tokenizer("(add 1 4)");

    println!("{:?}", tokens);
}
