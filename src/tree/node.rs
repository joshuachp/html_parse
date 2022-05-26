#[derive(Debug, Clone, Copy)]
pub struct Node<T> {
    parent: Option<usize>,
    prev_sibling: Option<usize>,
    next_sibling: Option<usize>,
    child: Option<usize>,
    pub value: T,
}

impl<T> Node<T> {
    /// Create a new node
    pub fn new(value: T) -> Self {
        Self {
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            child: None,
            value,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Iter<'a, T> {
    node: Option<&'a Node<T>>,
    range: &'a [Node<T>],
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;

        let next = if let Some(child) = node.child {
            self.range.get(child)
        } else if let Some(sibling) = node.next_sibling {
            self.range.get(sibling)
        } else if let Some(parent) = node.parent {
            let next = self.range.get(parent)?.next_sibling?;

            self.range.get(next)
        } else {
            None
        };

        self.node = next;

        Some(&node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_iter_empty() {
        let mut iter: Iter<i32> = Iter {
            node: None,
            range: &[],
        };

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_iter() {
        let node = Node::new(1);

        let mut iter = Iter {
            node: Some(&node),
            range: &[node],
        };

        assert_eq!(iter.next().map(|t| *t), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_iter_two() {
        let mut node_1 = Node::new(1);
        let node_2 = Node::new(2);

        node_1.child = Some(1);

        let mut iter = Iter {
            node: Some(&node_1),
            range: &[node_1, node_2],
        };

        assert_eq!(iter.next().map(|t| *t), Some(1));
        assert_eq!(iter.next().map(|t| *t), Some(2));
        assert_eq!(iter.next(), None);
    }
}
