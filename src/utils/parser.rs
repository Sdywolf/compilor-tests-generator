use crate::utils::ast::Tree;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

pub fn get_tree(input: &str) -> Tree {
    let tree = parser::StartParser::new().parse(input).unwrap();
    tree
}
