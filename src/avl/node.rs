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

    pub fn search<'a>(node: &'a mut Box<Node<K, V>>, key: &K) -> Option<&'a mut Box<Node<K, V>>> {
        if *key < node.index {
            if let Some(child) = node.left.as_mut() {
                return Node::search(child, key);
            }
            return None;
        } else if *key > node.index {
            if let Some(child) = node.right.as_mut() {
                return Node::search(child, key);
            }
            return None;
        }
        return Some(node);
    }

    pub fn search_child_to_delete(
        mut node: Box<Node<K, V>>,
        key: &K,
    ) -> Result<Box<Node<K, V>>, Box<Node<K, V>>> {
        if *key < node.index {
            if let Some(mut child) = node.left.take() {
                if child.index == *key {
                    node.left = Node::find_inorder_successor(&mut child);
                    if node.left.is_some() {
                        return Ok(Node::check_balance(node));
                    }
                    return Ok(node);
                } else {
                    return Node::search_child_to_delete(child, key);
                }
            }
            return Err(node);
        } else if *key > node.index {
            if let Some(mut child) = node.right.take() {
                if child.index == *key {
                    node.right = Node::find_inorder_successor(&mut child);
                    if node.right.is_some() {
                        return Ok(Node::check_balance(node));
                    }
                    return Ok(node);
                } else {
                    return Node::search_child_to_delete(child, key);
                }
            }
            return Err(node);
        }
        panic!("node index was equal to key in search_child_to_delete");
    }

    pub fn find_inorder_successor(node: &mut Box<Node<K, V>>) -> Option<Box<Node<K, V>>> {
        if node.left.is_none() && node.right.is_none() {
            return None;
        }
        if node.right.is_none() {
            return node.left.take();
        }
        let mut right_node = node.right.take().unwrap();
        if right_node.left.is_none() && right_node.right.is_none() {
            return Some(right_node);
        }
        if right_node.right.is_none() {
            return Some(right_node.right.unwrap());
        }
        Some(Node::retrieve_successor_from_child(&mut right_node).unwrap())
    }

    pub fn retrieve_successor_from_child(
        node: &mut Box<Node<K, V>>,
    ) -> Result<Box<Node<K, V>>, ()> {
        if let Some(mut child) = node.left.as_mut() {
            if child.left.is_some() || child.right.is_some() {
                return Node::retrieve_successor_from_child(&mut child);
            }
            let successor = node.left.take();
            return Ok(successor.unwrap());
        }
        if let Some(mut child) = node.right.as_mut() {
            if child.left.is_some() || child.right.is_some() {
                return Node::retrieve_successor_from_child(&mut child);
            }
            let successor = node.right.take();
            return Ok(successor.unwrap());
        }

        Err(())
    }

    pub fn insert(mut node: Box<Node<K, V>>, insert_item: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if insert_item.index < node.index {
            if let Some(mut child) = node.left {
                child = Node::insert(child, insert_item);
                node.left = Some(child);
            } else {
                node.left = Some(insert_item);
            }
            return Node::check_balance(node);
        } else {
            if let Some(mut child) = node.right {
                child = Node::insert(child, insert_item);
                node.right = Some(child);
            } else {
                node.right = Some(insert_item);
            }
            return Node::check_balance(node);
        }
    }
    pub fn check_balance(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let left_height = Node::find_left_height(&node);
        let right_height = Node::find_right_height(&node);

        if left_height - right_height > 1 {
            let child = node.left.as_ref().unwrap();
            let left = Node::find_left_height(&child);
            let right = Node::find_right_height(&child);

            if left > right {
                /*leftleft rotate right */
                return Node::right(node);
            } else {
                /*leftright rotate  left then right */
                let mut child = node.left.take().unwrap();
                child = Node::left(child);
                return Node::right(child);
            }
        } else if right_height - left_height > 1 {
            let child = node.right.as_ref().unwrap();
            let left = Node::find_left_height(&child);
            let right = Node::find_right_height(&child);

            if right > left {
                /*rightright rotate left*/
                return Node::left(node);
            } else {
                /*rightleft	rotate right then left */
                let mut child = node.right.take().unwrap();
                child = Node::right(child);
                return Node::left(child);
            }
        }
        return node;
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
