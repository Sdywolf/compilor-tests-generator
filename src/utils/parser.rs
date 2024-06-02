use crate::utils::ast::{Class, Node, Tree};

pub fn get_tree(input: &str) -> Option<Box<Tree>> {
    let tree = parser::start(input);
    tree.unwrap()
}

peg::parser! {
    grammar parser() for str {
        pub rule start() -> Option<Box<Tree>> = _ t:tree() _ quiet!{[';' | '.']} _ { t }

        rule tree() -> Option<Box<Tree>> =
            "nil" { None } /
            "t(" _ node:node() _ "," _ ltree:tree() _ "," _ rtree:tree() _ ")" {
                Some(Box::new(Tree { node, ltree, rtree }))
            }

        rule node() -> Node = "n(" _ id:number() _ "," _ class:class() _ ")" {
            Node { id, class }
        }

        rule number() -> i32 = n:$(['0' | '1'..='9']['0'..='9']*) {
            n.parse().unwrap()
        }

        rule class() -> Class =
            "def" { Class::Def } /
            "use" { Class::Use } /
            "block" { Class::Block }

        rule _ = quiet!{[' ' | '\t' | '\n' | '\r']*}
    }
}
