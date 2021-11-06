#[cfg(test)]
mod tests {
    use crate::avl::AvlTree;
    #[test]
    fn insertion_retrieval() {
        let mut tree = AvlTree::<i32, i32>::new();
        tree.insert(1, 1);
        tree.insert(3, 3);
        tree.insert(2, 2);
        tree.insert(4, 4);
        assert_eq!(tree.get_as_mut(&1), Some(&mut 1));
        assert_eq!(tree.get_as_mut(&2), Some(&mut 2));
        assert_eq!(tree.get_as_mut(&3), Some(&mut 3));
        assert_eq!(tree.get_as_mut(&4), Some(&mut 4));
        assert_eq!(tree.get_as_ref(&1), Some(&1));
        assert_eq!(tree.get_as_ref(&2), Some(&2));
        assert_eq!(tree.get_as_ref(&3), Some(&3));
        assert_eq!(tree.get_as_ref(&4), Some(&4));
        assert_eq!(tree.get_as_mut(&5), None);
        assert_eq!(tree.get_as_ref(&5), None)
    }

    #[test]
    fn single_lvl_rotation() {
        let mut tree = AvlTree::<i32, i32>::new();
        tree.insert(1, 1);
        tree.insert(3, 3);
        tree.insert(5, 5);
        assert_eq!(tree.get_root().unwrap().index, 3);

        let mut tree = AvlTree::<i32, i32>::new();
        tree.insert(5, 5);
        tree.insert(3, 3);
        tree.insert(1, 1);
        assert_eq!(tree.get_root().unwrap().index, 3);
    }

    #[test]
    fn double_lvl_rotation() {
        let mut tree = AvlTree::<i32, i32>::new();
        tree.insert(100, 100);
        tree.insert(150, 150);
        tree.insert(50, 50);
        tree.insert(25, 25);
        tree.insert(75, 75);
        tree.insert(60, 60);
        assert_eq!(tree.get_root().unwrap().index, 75);
        assert_eq!(
            tree.get_root()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .index,
            60
        );
        let mut tree = AvlTree::<i32, i32>::new();
        tree.insert(100, 100);
        tree.insert(150, 150);
        tree.insert(50, 50);
        tree.insert(125, 125);
        tree.insert(175, 175);
        tree.insert(140, 140);
        assert_eq!(tree.get_root().unwrap().index, 125);
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
            140
        );
    }
    #[test]
    fn deletion() {
        let mut tree = AvlTree::<i32, i32>::new();
        tree.insert(100, 100);
        tree.insert(150, 150);
        tree.insert(50, 50);
        tree.insert(25, 25);
        tree.insert(75, 75);
        tree.insert(125, 125);
        tree.insert(175, 175);
        tree.insert(12, 12);
        tree.insert(37, 37);
        tree.insert(63, 63);
        tree.insert(87, 87);
        tree.insert(112, 112);
        tree.insert(137, 137);
        tree.insert(163, 163);
        tree.insert(187, 187);

        assert_eq!(tree.delete(&-10), Err(()));
        tree.delete(&12).unwrap();
        tree.delete(&25).unwrap();
        tree.delete(&100).unwrap();
        assert_eq!(tree.get_root().unwrap().index, 112);
        tree.delete(&112).unwrap();
        assert_eq!(tree.get_root().unwrap().index, 125);
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
            137
        );
        tree.delete(&137).unwrap();
        assert_eq!(tree.get_root().unwrap().index, 125);
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
            150
        );
    }
}
