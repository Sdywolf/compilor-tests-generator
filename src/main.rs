mod utils;

use utils::parser;

const ENGINE: &str = include_str!("../generator/gen.pl");

const QUERY: &str = r#"
main :- def_use(T, 3), portray_clause(T).
:- initialization(main).
"#;

fn main() {
    let code = format!("{ENGINE}\n{QUERY}");
    let result = utils::engine::eval_code(&code);
    println!("{}", result);
    let tree = parser::get_tree(&result);
    println!("{:?}", tree);
}
