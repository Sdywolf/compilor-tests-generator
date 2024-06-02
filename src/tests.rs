use super::*;

#[test]
fn test_parser() {
    let input = "t(n(0,block),t(n(1,def),nil,t(n(1,use),nil,nil)), nil).";
    let tree = utils::parser::get_tree(input);
    let s = SimpleAST::from_tree(tree);
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].to_string(), "{    decl x1;\n\n    use x1;\n\n}");
}
