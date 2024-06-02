mod utils;

use crate::utils::printer::SimpleAST;

const ENGINE: &str = include_str!("../generator/gen.pl");

const QUERY: &str = r#"
main :- ###QUERY###, portray_clause(T).
:- initialization(main).
"#;

#[cfg(test)]
mod tests;

/// example:
///
/// ```text
/// cargo run -- "def_use(T, 3)"
/// ```
fn main() {
    let input = std::env::args().nth(1).expect("missing argument");
    let query = QUERY.replace("###QUERY###", &input);
    let code = format!("{ENGINE}\n{query}");

    let result = utils::engine::eval_code(&code);
    dbg!(&result);

    let tree = utils::parser::get_tree(&result);
    let stmts = SimpleAST::from_tree(tree);
    for stmt in stmts {
        println!("{}", stmt);
    }
}
