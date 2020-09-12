use crate::parser::{Expr, Program};

#[derive(Debug)]
pub enum CExpr {
    NumberLiteral(String),
    StringLiteral(String),
    Statement(Box<CExpr>),
    CallExpression(Box<CExpr>, Vec<CExpr>),
    Identifier(String),
}

#[derive(Debug)]
pub struct CProgram {
    body: Vec<CExpr>,
}

fn traverse_node(node: &Expr, parent: Option<&Expr>) -> CExpr {
    match node {
        Expr::NumberLiteral(value) => CExpr::NumberLiteral(value.to_string()),
        Expr::StringLiteral(value) => CExpr::StringLiteral(value.to_string()),
        Expr::CallExpression(name, params) => {
            let call_expression = CExpr::CallExpression(
                Box::new(CExpr::Identifier(name.to_string())),
                traverse_nodes(params, Some(node)),
            );

            let statement = match parent {
                Some(&Expr::CallExpression(_, _)) => call_expression,
                _ => CExpr::Statement(Box::new(call_expression)),
            };

            return statement;
        }
    }
}

fn traverse_nodes(nodes: &Vec<Expr>, parent: Option<&Expr>) -> Vec<CExpr> {
    nodes
        .iter()
        .map(|elem| traverse_node(elem, parent))
        .collect()
}

pub fn c_ast_from_program(ast: Program) -> CProgram {
    let body = traverse_nodes(&ast.body, None);

    CProgram { body: body }
}
