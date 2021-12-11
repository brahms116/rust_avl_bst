use super::AvlTree;
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
	end: Option<&'a K>,
	stack: Vec<&'a mut Box<Node<K, V>>>,
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
			stack: vec![],
		};
		if let Some(node) = tree.get_root() {
			iter.stack.push(node)
		}
		iter
	}
}

impl<'a, K: PartialOrd, V> Iterator for InorderIteratorRefMut<'a, K, V> {
	type Item = &'a mut V;
	fn next(&mut self) -> Option<&'a mut V> {
		let len = self.stack.len();
		if len == 0 {
			return None;
		}
		let mut node = self.stack.pop().unwrap();
		if let Some(left_node) = node.left.as_mut() {
			self.stack.push(node);
			self.stack.push(left_node);
			return self.next();
		}
		let result = Some(&mut node.value);
		if let Some(right_node) = node.right.as_mut() {
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
