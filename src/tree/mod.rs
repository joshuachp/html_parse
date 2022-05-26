//! Tree component

use self::node::Node;

pub mod node;

#[derive(Debug, Clone)]
pub struct Tree<T> {
    root: usize,
    tail: usize,
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    #[must_use]
    pub fn new(node: Node<T>) -> Self {
        Self {
            root: 0,
            tail: 0,
            nodes: vec![node],
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get<'a>(&'a self, index: usize) -> Option<&'a Node<T>> {
        self.nodes.get(index)
    }

    pub fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut Node<T>> {
        self.nodes.get_mut(index)
    }
}
