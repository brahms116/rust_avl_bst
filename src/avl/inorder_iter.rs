use super::AvlTree;
use super::Branch;
use super::Node;

pub struct InorderIterator<'a, K: PartialOrd, V> {
    start: Option<&'a K>,
    end: Option<&'a K>,
    stack: Vec<Box<Node<K, V>>>,
}

pub struct InorderIteratorRef<'a, K: PartialOrd, V> {
    tree: &'a AvlTree<K, V>,
}

pub struct InorderIteratorRefMut<'a, K: PartialOrd, V> {
    start: Option<&'a K>,
    tree: &'a mut AvlTree<K, V>,
    end: Option<&'a K>,
    is_ptr_down: bool,
    stack: Vec<Branch>,
}

impl<'a, K: PartialOrd, V> InorderIteratorRefMut<'a, K, V> {
    pub fn new(
        tree: &'a mut AvlTree<K, V>,
        start: Option<&'a K>,
        end: Option<&'a K>,
    ) -> InorderIteratorRefMut<'a, K, V> {
        let mut iter = InorderIteratorRefMut {
            start,
            end,
            tree,
            is_ptr_down: true,
            stack: vec![],
        };
        iter
    }
}

// impl<'a, K: PartialOrd, V> Iterator for InorderIteratorRefMut<'a, K, V> {
// 	type Item = &'a mut V;

// 	fn next(&mut self) -> Option<&'a mut V> {
// 		let root = self.tree.get_root();

// 		if root.is_none() {
// 			return None;
// 		}

// 		let root = root.unwrap();

// 		if self.stack.is_empty() && !self.is_ptr_down {
// 			return None;
// 		}

// 		return Some(&mut root.value);

// 		// let root: &mut Box<Node<K, V>> = self.root.as_mut().unwrap();

// 		// let curr_node: Option<&'a mut Box<Node<K, V>>> = Some(root);

// 		// let curr_node = Node::get_from_directions(&mut self.root, &self.stack);

// 		/* If for some reason the stack points to an invalid node */
// 		// if curr_node.is_none() {
// 		// 	return None;
// 		// }

// 		// let curr_node: &'a mut Box<Node<K, V>> = curr_node.unwrap();

// 		None

// 		// return Some(&mut curr_node.value);
// 		// let mut result: Option<&'a mut V> = None;

// 		// if self.is_ptr_down {
// 		// 	if curr_node.left.is_some() {
// 		// 		self.stack.push(Branch::Left);
// 		// 		return self.next();
// 		// 	}
// 		// 	result = Some(&mut curr_node.value);
// 		// 	if curr_node.right.is_some() {
// 		// 		self.stack.push(Branch::Right)
// 		// 	} else {
// 		// 	}
// 		// }
// 		// return result;
// 	}
// }

impl<'a, K: PartialOrd, V> InorderIterator<'a, K, V> {
    pub fn new(
        tree: AvlTree<K, V>,
        start: Option<&'a K>,
        end: Option<&'a K>,
    ) -> InorderIterator<'a, K, V> {
        let mut iter = InorderIterator {
            start,
            end,
            stack: vec![],
        };
        if let Some(node) = tree.take_root() {
            iter.stack.push(node)
        }
        iter
    }
}

impl<'a, K: PartialOrd, V> Iterator for InorderIterator<'a, K, V> {
    type Item = V;
    fn next(&mut self) -> Option<V> {
        let len = self.stack.len();
        if len == 0 {
            return None;
        }
        let mut node = self.stack.pop().unwrap();
        if let Some(left_node) = node.left.take() {
            self.stack.push(node);
            self.stack.push(left_node);
            return self.next();
        }
        let result = Some(node.value);
        if let Some(right_node) = node.right.take() {
            self.stack.push(right_node)
        }

        if let Some(min) = self.start.as_ref() {
            if node.index < **min {
                return self.next();
            }
        }
        if let Some(max) = self.end.as_ref() {
            if node.index >= **max {
                return None;
            }
        }
        result
    }
}
