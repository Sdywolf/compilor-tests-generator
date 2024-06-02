#[derive(Copy, Clone)]
pub enum Class {
    Def,
    Use,
    Block
}

#[derive(Copy, Clone)]
pub struct Node {
    id : i32, 
    class : Class
}

#[derive(Clone)]
pub struct Tree {
    node : Node, 
    ltree : Box<Tree>, 
    rtree : Box<Tree>
}