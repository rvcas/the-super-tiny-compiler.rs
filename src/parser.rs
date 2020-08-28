use std::iter::Peekable;
use std::vec::IntoIter;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum AST {
    Program(Box<AST>),
    NumberLiteral(String),
    StringLiteral(String),
    CallExpression(String, Vec<AST>),
}

fn walk(tokens: &mut Peekable<IntoIter<Token>>) -> AST {
    if let Some(token) = tokens.next() {
        return match token {
            Token::Number(value) => AST::NumberLiteral(String::from(value)),
            Token::TString(value) => AST::StringLiteral(String::from(value)),
            Token::Paren(value) if value == '(' => {
                if let Some(next) = tokens.next() {
                    match next {
                        Token::Name(name) => {
                            let mut params: Vec<AST> = Vec::new();

                            while let Some(param) = tokens.peek() {
                                match param {
                                    Token::Paren(paren) if paren == &')' => break,
                                    _ => params.push(walk(tokens)),
                                }
                            }

                            let _ = tokens.next();

                            return AST::CallExpression(String::from(name), params);
                        }
                        _ => panic!("Expected CallExpression after ("),
                    }
                } else {
                    panic!("Expected CallExpression after (");
                }
            }
            _ => unreachable!(),
        };
    } else {
        unreachable!();
    };
}

pub fn ast_from_tokens(tks: Vec<Token>) -> AST {
    let mut tokens = tks.into_iter().peekable();

    AST::Program(Box::new(walk(&mut tokens)))
}
