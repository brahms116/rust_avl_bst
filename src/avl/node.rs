pub struct Node<K, V>
where
	K: PartialOrd,
{
	pub index: K,
	pub value: V,
	pub left: Option<Box<Node<K, V>>>,
	pub right: Option<Box<Node<K, V>>>,
}

impl<K: PartialOrd, V> Node<K, V> {
	pub fn new(index: K, value: V) -> Node<K, V> {
		Node {
			index,
			value,
			left: None,
			right: None,
		}
	}

	pub fn right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
		let mut left = node.left.unwrap();
		let left_right_opt = left.right.take();
		node.left = left_right_opt;
		left.right = Some(node);
		return left;
	}

	pub fn left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
		let mut right = node.right.unwrap();
		let right_left_opt = right.left.take();
		node.right = right_left_opt;
		right.left = Some(node);
		return right;
	}

	pub fn find_right_height(node: &Box<Node<K, V>>) -> i32 {
		if let Some(child) = node.right.as_ref() {
			return 1 + Node::find_height(child);
		}
		1
	}
	pub fn find_left_height(node: &Box<Node<K, V>>) -> i32 {
		if let Some(child) = node.left.as_ref() {
			return 1 + Node::find_height(child);
		}
		1
	}
	pub fn find_height(node: &Box<Node<K, V>>) -> i32 {
		let left_height = Node::find_left_height(node);
		let right_height = Node::find_right_height(node);

		if left_height > right_height {
			left_height
		} else {
			right_height
		}
	}
}
