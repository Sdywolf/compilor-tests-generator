use std::fmt::Display;

use crate::utils::ast::{Class, Node, Tree};

#[allow(dead_code)]
pub enum SimpleAST {
    Decl(i32),
    Use(i32),
    Loop(Vec<SimpleAST>),
    Block(Vec<SimpleAST>),
    Break,
}

impl SimpleAST {
    pub fn from_tree(tree: Option<Box<Tree>>) -> Vec<Self> {
        match tree {
            None => vec![],
            Some(tree) => {
                let Tree { node, ltree, rtree } = *tree;
                let mut result = match node {
                    Node {
                        id,
                        class: Class::Def,
                    } => vec![SimpleAST::Decl(id)],
                    Node {
                        id,
                        class: Class::Use,
                    } => vec![SimpleAST::Use(id)],
                    Node {
                        class: Class::Block,
                        ..
                    } => vec![SimpleAST::Block(SimpleAST::from_tree(ltree))],
                };
                result.extend(SimpleAST::from_tree(rtree));
                result
            }
        }
    }
}

impl Display for SimpleAST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleAST::Decl(id) => write!(f, "decl x{};", id),
            SimpleAST::Use(id) => write!(f, "use x{};", id),
            SimpleAST::Loop(body) => {
                write!(f, "loop {{")?;
                for ast in body {
                    writeln!(f, "{}", Indent { inner: ast })?;
                }
                write!(f, "}}")
            }
            SimpleAST::Block(body) => {
                write!(f, "{{")?;
                for ast in body {
                    writeln!(f, "{}", Indent { inner: ast })?;
                }
                write!(f, "}}")
            }
            SimpleAST::Break => write!(f, "break;"),
        }
    }
}

struct Indent<T> {
    pub inner: T,
}

impl<T> Display for Indent<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = format!("{}", self.inner);
        for line in inner.split('\n') {
            writeln!(f, "    {}", line)?;
        }
        Ok(())
    }
}
