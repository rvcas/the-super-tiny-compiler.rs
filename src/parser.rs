use crate::tokenizer::Token;

#[derive(Debug)]
pub struct AST {}

pub fn ast_from_tokens(tks: &[Token]) -> AST {
    let mut _tokens = tks.into_iter().peekable();

    AST {}
}
