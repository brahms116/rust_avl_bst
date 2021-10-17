mod primitive_test;

pub struct PrimitiveNode {
	pub index: u32,
	pub left: Option<Box<PrimitiveNode>>,
	pub right: Option<Box<PrimitiveNode>>,
}

impl PrimitiveNode {
	pub fn new(i: u32) -> PrimitiveNode {
		PrimitiveNode {
			index: i,
			left: None,
			right: None,
		}
	}

	pub fn right(mut item: Box<PrimitiveNode>) -> Box<PrimitiveNode> {
		let mut left = item.left.unwrap();
		let left_right_opt = left.right.take();
		item.left = left_right_opt;
		left.right = Some(item);
		return left;
	}

	// pub fn delete(mut item: Box<PrimitiveNode>) -> Box<PrimitiveNode> {
	/* Can't complete this function with this design */
	/* check if left child and right child exists */
	/* In order accessor can be found by the leftest most node on the right side of this node */
	// }

	pub fn left(mut item: Box<PrimitiveNode>) -> Box<PrimitiveNode> {
		let mut right = item.right.unwrap();
		let right_left_opt = right.left.take();
		item.right = right_left_opt;
		right.left = Some(item);
		return right;
	}

	pub fn check_balance(mut item: Box<PrimitiveNode>) -> Box<PrimitiveNode> {
		let left_height = item.find_left_height();
		let right_height = item.find_right_height();

		if left_height - right_height > 1 {
			let child = item.left.as_ref().unwrap();
			let left = child.find_left_height();
			let right = child.find_right_height();

			if left > right {
				/*leftleft rotate right */
				return PrimitiveNode::right(item);
			} else {
				/*leftright rotate  left then right */
				let mut child = item.left.take().unwrap();
				child = PrimitiveNode::left(child);
				return PrimitiveNode::right(child);
			}
		} else if right_height - left_height > 1 {
			let child = item.right.as_ref().unwrap();
			let left = child.find_left_height();
			let right = child.find_right_height();

			if right > left {
				/*rightright rotate left*/
				return PrimitiveNode::left(item);
			} else {
				/*rightleft	rotate right then left */
				let mut child = item.right.take().unwrap();
				child = PrimitiveNode::right(child);
				return PrimitiveNode::left(child);
			}
		}
		return item;
	}

	pub fn insert_item(mut root: Box<PrimitiveNode>, item: PrimitiveNode) -> Box<PrimitiveNode> {
		if item.index < root.index {
			if let Some(child) = root.left {
				root.left = Some(PrimitiveNode::insert_item(child, item));
				root = PrimitiveNode::check_balance(root)
			} else {
				root.left = Some(Box::new(item));
			}
		} else {
			if let Some(child) = root.right {
				root.right = Some(PrimitiveNode::insert_item(child, item));
				root = PrimitiveNode::check_balance(root)
			} else {
				root.right = Some(Box::new(item));
			}
		}
		return root;
	}

	pub fn search(&mut self, index: u32) -> Option<&mut PrimitiveNode> {
		if self.index == index {
			return Some(self);
		}

		if index < self.index {
			if let Some(child) = self.left.as_mut() {
				return child.search(index);
			}
		}
		if index > self.index {
			if let Some(child) = self.right.as_mut() {
				return child.search(index);
			}
		}
		return None;
	}

	pub fn find_height(&self) -> i32 {
		let left_height = self.find_left_height();
		let right_height = self.find_right_height();

		if left_height > right_height {
			left_height
		} else {
			right_height
		}
	}

	pub fn find_right_height(&self) -> i32 {
		if let Some(child) = self.right.as_ref() {
			return 1 + child.find_height();
		}
		1
	}

	pub fn find_left_height(&self) -> i32 {
		if let Some(child) = self.left.as_ref() {
			return 1 + child.find_height();
		}
		1
	}
}
