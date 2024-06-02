use super::*;

#[test]
fn test_parser() {
    let input = "t(n(0,block),t(n(1,def),nil,t(n(1,use),nil,nil)), nil)";
    let tree = parser::get_tree(&input);
    let s = dump::dump(&tree);
    assert_eq!(s, "{Def 1;Use 1;}");
}