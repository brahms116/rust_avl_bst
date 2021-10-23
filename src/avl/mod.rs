pub mod node;

use node::Node;
pub struct AvlTree<K: PartialOrd, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: PartialOrd, V> AvlTree<K, V> {
    pub fn new() -> AvlTree<K, V> {
        AvlTree { root: None }
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some(root) = self.root.as_mut() {
        } else {
            self.root = Some(Box::new(Node::new(k, v)))
        }
    }
}
