use crate::transformer::{CExpr, CProgram};

fn build_statement(node: &CExpr) -> String {
    match node {
        CExpr::Statement(expression) => build_statement(expression) + ";",
        CExpr::CallExpression(callee, arguments) => {
            build_statement(callee)
                + "("
                + &arguments
                    .iter()
                    .map(|arg| build_statement(arg))
                    .collect::<Vec<String>>()
                    .join(", ")
                + ")"
        }
        CExpr::Identifier(name) => name.to_string(),
        CExpr::NumberLiteral(value) => value.to_string(),
        CExpr::StringLiteral(value) => value.to_string(),
    }
}

pub fn c_ast_to_string(c_ast: CProgram) -> String {
    c_ast
        .body
        .iter()
        .map(|statement| build_statement(statement))
        .collect::<Vec<String>>()
        .join("\n")
}
