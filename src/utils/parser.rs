use lalrpop_util::lalrpop_mod;
use std::io;
use crate::utils::ast::{Tree, Node, Class};

lalrpop_mod!(parser);

pub fn get_tree(input: &str) -> Tree {
    let tree = parser::TreeParser::new().parse(input).unwrap();
    tree
}