mod utils;
use utils::{parser, ast};
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tree = parser::get_tree(&input);
}
