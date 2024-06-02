use super::ast::{Tree, Node, Class};

pub fn dumptree(t : &Tree) -> String{
    let mut s = String::new();
    match t.node.class {
        Class::Def => {
            s.push_str(&format!("Def {};", t.node.id));
            // println!("Def {};", t.node.id);
        },
        Class::Use => {
            s.push_str(&format!("Use {};", t.node.id));
            // println!("Use {};", t.node.id);
        },
        Class::Block => {
            s.push_str(&format!("{{"));
            // println!("{}", '{');
        }
    }
    match t.ltree {
        Some(ref ltree) => {
            s.push_str(dumptree(ltree).as_str());
        },
        None => {}
    }
    match t.node.class {
        Class::Block => {
            s.push_str(&format!("}}"));
            // println!("{}", '}');
        },
        _ => {}
    }
    match t.rtree {
        Some(ref rtree) => {
            s.push_str(dumptree(rtree).as_str());
        },
        None => {}
    }
    s
}

pub fn dump(t : &Option<Box<Tree> >) -> String {
    match t {
        Some(ref tree) => {
            dumptree(tree)
        },
        None => {"".to_string()}
    }
}