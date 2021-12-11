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
	root: Option<&'a mut Box<Node<K, V>>>,
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
			root: None,
			is_ptr_down: true,
			stack: vec![],
		};
		iter.root = tree.get_root();
		iter
	}
}

impl<'a, K: PartialOrd, V> Iterator for InorderIteratorRefMut<'a, K, V> {
	type Item = &'a mut V;

	fn next(&mut self) -> Option<&'a mut V> {
		if self.root.is_none() {
			return None;
		}

		if self.stack.is_empty() && !self.is_ptr_down {
			return None;
		}

		let root = self.root.as_mut().unwrap();
		let curr_node = Node::get_from_directions(root, &self.stack);

		/* If for some reason the stack points to an invalid node */
		if curr_node.is_none() {
			return None;
		}

		let curr_node = curr_node.unwrap();

		let result: Option<&'a mut V> = None;

		if self.is_ptr_down {
			if curr_node.left.is_some() {
				self.stack.push(Branch::Left);
				return self.next();
			}
			result = Some(&mut curr_node.value);
			if curr_node.right.is_some() {
				self.stack.push(Branch::Right)
			} else {
			}
		}
		result
	}
}

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
