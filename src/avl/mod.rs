mod avl_tests;
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
        if let Some(mut root) = self.root.take() {
            root = Node::insert(root, Box::new(Node::new(k, v)));
            self.root = Some(root);
        } else {
            self.root = Some(Box::new(Node::new(k, v)))
        }
    }

    pub fn get_as_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(root) = self.root.as_mut() {
            if let Some(node) = Node::search(root, key) {
                return Some(&mut node.value);
            }
            return None;
        }
        None
    }
    pub fn get_as_ref(&mut self, key: &K) -> Option<&V> {
        if let Some(root) = self.root.as_mut() {
            if let Some(node) = Node::search(root, key) {
                return Some(&node.value);
            }
            return None;
        }
        None
    }

    pub fn delete(&mut self, key: &K) -> Result<(), ()> {
        if let Some(node) = self.root.as_mut() {
            if node.index == *key {
                self.root = Node::find_inorder_successor(node);
                return Ok(());
            } else {
                return Node::search_child_to_delete(node, key);
            }
        }
        Err(())
    }
    pub fn get_root(&mut self) -> Option<&mut Box<Node<K, V>>> {
        self.root.as_mut()
    }
}
