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

    pub fn find_replacement(mut node: Box<Node<K, V>>) -> Option<Box<Node<K, V>>> {
        if node.left.is_none() && node.right.is_none() {
            return None;
        }
        if node.right.is_none() {
            return node.left.take();
        }
        let mut right_node = node.right.take().unwrap();
        if right_node.left.is_none() && right_node.right.is_none() {
            return Some(Node::replace_node(node, right_node));
        }
        let result = Node::retrieve_successor_from_child(&mut right_node).unwrap();
        node.right = Some(Node::check_balance(right_node));
        return Some(Node::replace_node(node, result));
    }

    pub fn replace_node(
        mut node: Box<Node<K, V>>,
        mut replacement: Box<Node<K, V>>,
    ) -> Box<Node<K, V>> {
        replacement.left = node.left.take();
        replacement.right = node.right.take();
        return replacement;
    }

    pub fn retrieve_successor_from_child(
        node: &mut Box<Node<K, V>>,
    ) -> Result<Box<Node<K, V>>, ()> {
        if let Some(mut child) = node.left.take() {
            if child.left.is_some() {
                let result = Node::retrieve_successor_from_child(&mut child);
                node.left = Some(Node::check_balance(child));
                return result;
            }
            node.left = child.right.take();
            return Ok(child);
        }
        Err(())
    }

    pub fn search_child_to_delete(
        mut node: Box<Node<K, V>>,
        key: &K,
    ) -> Result<Box<Node<K, V>>, Box<Node<K, V>>> {
        /* If key is smaller, check left child */
        if *key < node.index {
            /* Checks left child exists */
            if let Some(child) = node.left.take() {
                /* If left child is the item to be deleted */
                if child.index == *key {
                    println!("left child is key");
                    let replacement = Node::find_replacement(child);

                    /* If the replacement is not None, ie bottom leaf */
                    if let Some(rep) = replacement {
                        node.left = Some(Node::check_balance(rep));
                        return Ok(Node::check_balance(node));
                    }
                    /* Replacement is None, bottom leaf */
                    return Ok(Node::check_balance(node));
                }
                /* Left child is not the item to delete, dig deeper */
                else {
                    match Node::search_child_to_delete(child, key) {
                        Ok(res) => {
                            node.left = Some(res);
                            return Ok(Node::check_balance(node));
                        }
                        Err(res) => {
                            node.left = Some(res);
                            return Err(node);
                        }
                    }
                }
            }
            /*Left child doesn't exist */
            return Err(node);
        }
        /* Check right child */
        else if *key > node.index {
            if let Some(child) = node.right.take() {
                if child.index == *key {
                    let replacement = Node::find_replacement(child);
                    if let Some(rep) = replacement {
                        node.left = Some(Node::check_balance(rep));
                        return Ok(Node::check_balance(node));
                    }
                    return Ok(Node::check_balance(node));
                } else {
                    match Node::search_child_to_delete(child, key) {
                        Ok(res) => {
                            node.right = Some(res);
                            return Ok(Node::check_balance(node));
                        }
                        Err(res) => {
                            node.right = Some(res);
                            return Err(node);
                        }
                    }
                }
            }
            return Err(node);
        }
        panic!("node index was equal to key in search_child_to_delete");
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

            if left >= right {
                /*leftleft rotate right */
                return Node::right(node);
            } else {
                /*leftright rotate left then right */
                let child = node.left.take().unwrap();
                node.left = Some(Node::left(child));
                return Node::right(node);
            }
        } else if right_height - left_height > 1 {
            let child = node.right.as_ref().unwrap();
            let left = Node::find_left_height(&child);
            let right = Node::find_right_height(&child);

            if right >= left {
                /*rightright rotate left*/
                return Node::left(node);
            } else {
                /*rightleft	rotate right then left */
                let child = node.right.take().unwrap();
                node.right = Some(Node::right(child));
                return Node::left(node);
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
