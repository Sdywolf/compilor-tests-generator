#[derive(Copy, Clone)]
pub enum Class {
    Def,
    Use,
    Block
}

#[derive(Copy, Clone)]
pub struct Node {
    pub id : i32, 
    pub class : Class
}

#[derive(Clone)]
pub struct Tree {
    pub node : Node, 
    pub ltree : Box<Tree>, 
    pub rtree : Box<Tree>
}