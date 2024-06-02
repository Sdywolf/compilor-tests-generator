mod utils;

use utils::{parser, dump};

use crate::utils::printer::SimpleAST;

const ENGINE: &str = include_str!("../generator/gen.pl");

const QUERY: &str = r#"
main :- def_use(T, 3), portray_clause(T).
:- initialization(main).
"#;

#[cfg(test)]
mod tests;

fn main() {
    let code = format!("{ENGINE}\n{QUERY}");
    let result = utils::engine::eval_code(&code);
    let tree = parser::get_tree(&result);
    let stmts = SimpleAST::from_tree(tree);
    for stmt in stmts {
        println!("{}", stmt);
    }
}
