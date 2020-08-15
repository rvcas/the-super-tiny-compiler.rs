#[derive(Debug)]
enum Token {
    Paren(char),
    Number(String),
}

fn tokenizer(input: &str) -> Vec<Token> {
    let _current = 0;

    let mut tokens: Vec<Token> = Vec::new();

    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' => {}
            '(' | ')' => tokens.push(Token::Paren(ch)),
            '0'..='9' => {
                let mut value = String::new();

                value.push(ch);

                while let Some(sub) = chars.next() {
                    match sub {
                        '0'..='9' => value.push(sub),
                        _ => break,
                    }
                }

                tokens.push(Token::Number(value));
            }

            _ => panic!("I dont know what this character is: {}", ch),
        }
    }

    println!("{:?}", tokens);

    return tokens;
}

fn main() {
    tokenizer("(add 1 4)");
}
