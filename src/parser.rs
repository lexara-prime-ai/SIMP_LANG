use pest::{iterators::Pair, Parser};

use crate::ast::Node;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct LangParser;

pub fn parse(source: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast: Vec<Node> = vec![];
    let pairs = LangParser::parse(Rule::Program, source)?;

    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}

pub fn build_ast_from_expr(pair: Pair<'_, Rule>) -> Result<Node, anyhow::Error> {
    todo!()
}