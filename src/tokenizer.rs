#[derive(Debug)]
pub enum Token {
    Paren(char),
    Number(String),
    TString(String),
    Name(String),
}

pub fn from_str(input: &str) -> Vec<Token> {
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
                            let next = chars.next().unwrap();

                            value.push(next);
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
                            let _ = chars.next();
                            break;
                        }
                        _ => {
                            let next = chars.next().unwrap();

                            value.push(next);
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
                            let next = chars.next().unwrap();

                            value.push(next);
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
