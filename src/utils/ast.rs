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

#[derive(Clone)]
pub struct Tree {
    pub node : Node, 
    pub ltree : Option<Box<Tree> >, 
    pub rtree : Option<Box<Tree> >
}