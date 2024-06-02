use crate::utils::ast::Tree;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

pub fn get_tree(input: &str) -> Option<Box<Tree> > {
    let tree = parser::TreeParser::new().parse(input).unwrap();
    tree
}
