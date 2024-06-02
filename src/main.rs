mod utils;

use utils::parser;

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
    // println!("{}", result);
    input = "t(n(0,def),nil,t(n(0,use),nil,nil))".to_string();
    let tree = parser::get_tree(&result);
    dump::dump(&tree);
    println!("{:?}", tree);
}
