mod utils;
use utils::{parser, ast, dump};
use std::io;

#[cfg(test)]
mod tests;

fn main() {
    let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    input = "t(n(0,def),nil,t(n(0,use),nil,nil))".to_string();
    let tree = parser::get_tree(&input);
    dump::dump(&tree);
}
