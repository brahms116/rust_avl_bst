#[cfg(test)]
mod tests {
	use crate::primitive::PrimitiveNode;
	#[test]
	fn create() {
		let root = Box::new(PrimitiveNode::new(0));
		assert_eq!(root.index, 0);
	}
	#[test]
	fn left_insert() {
		let mut root_node = Box::new(PrimitiveNode::new(4));
		root_node = PrimitiveNode::insert_item(root_node, PrimitiveNode::new(2));
		assert_eq!(root_node.left.is_some(), true);
		assert_eq!(root_node.find_height(), 2);
		assert_eq!(root_node.left.unwrap().index, 2);
	}
	#[test]
	fn right_insert() {
		let mut root_node = Box::new(PrimitiveNode::new(4));
		root_node = PrimitiveNode::insert_item(root_node, PrimitiveNode::new(7));
		assert_eq!(root_node.right.is_some(), true);
		assert_eq!(root_node.find_height(), 2);
		assert_eq!(root_node.right.unwrap().index, 7);
	}
	#[test]
	fn case_left_left() {
		let mut root = Box::new(PrimitiveNode::new(5));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(4));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(3));
		assert_eq!(root.index, 4);
	}
	#[test]
	fn case_left_right() {
		let mut root = Box::new(PrimitiveNode::new(5));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(3));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(4));
		assert_eq!(root.index, 3);
	}
	#[test]
	fn case_right_right() {
		let mut root = Box::new(PrimitiveNode::new(5));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(6));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(7));
		assert_eq!(root.index, 6);
	}
	#[test]
	fn case_right_left() {
		let mut root = Box::new(PrimitiveNode::new(5));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(7));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(6));
		assert_eq!(root.index, 7);
	}
	#[test]
	fn balancing() {
		let mut root = Box::new(PrimitiveNode::new(6));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(3));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(9));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(2));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(8));
		root = PrimitiveNode::insert_item(root, PrimitiveNode::new(4));
		assert_eq!(root.index, 6)
	}
}
