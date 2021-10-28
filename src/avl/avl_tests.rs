#[cfg(test)]
mod tests {

    use crate::avl::AvlTree;

    #[test]
    fn general_test() {
        let mut tree = AvlTree::<i32, &str>::new();

        tree.insert(100, &"100");

        assert_eq!(
            tree.get_root().unwrap().index,
            100,
            "initial insertion failed"
        );

        tree.insert(50, &"50");

        assert_eq!(
            tree.get_root().unwrap().left.as_ref().unwrap().index,
            50,
            "Left insert failed"
        );

        tree.insert(150, &"150");

        assert_eq!(
            tree.get_root().unwrap().right.as_ref().unwrap().index,
            150,
            "Right insert failed"
        );

        tree.insert(25, &"25");
        tree.insert(75, &"75");
        tree.insert(60, &"60");
        assert_eq!(tree.get_root().unwrap().index, 50, "Rotation failed");

        assert_eq!(
            tree.get_root()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .index,
            60,
            "Rotation subtree failed"
        )
    }
}
