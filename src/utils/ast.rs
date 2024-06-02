#[derive(Debug, Copy, Clone)]
pub enum Class {
    Def,
    Use,
    Block,
}

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub id: i32,
    pub class: Class,
}

#[derive(Debug, Clone)]
pub enum Tree {
    Nil,
    Node {
        node: Node,
        ltree: Box<Tree>,
        rtree: Box<Tree>,
    },
}
