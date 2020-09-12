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

#[cfg(test)]
mod tests {
    use crate::tokenizer;
    use crate::tokenizer::Token;

    #[test]
    fn from_str() {
        let tokens = tokenizer::from_str("(add 12 12)");

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::Paren('(')));
        assert!(matches!(&tokens[1], Token::Name(name) if name.eq(&String::from("add"))));
        assert!(matches!(&tokens[2], Token::Number(number) if number.eq(&String::from("12"))));
        assert!(matches!(&tokens[3], Token::Number(number) if number.eq(&String::from("12"))));
        assert!(matches!(tokens[4], Token::Paren(')')));
    }
}
