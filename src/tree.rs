trait Set<T> {
    fn empty() -> Self;
    fn insert(&self, elf: T) -> Self;
    fn member(&self, elt: &T) -> bool;
}

#[derive(Clone)]
enum Tree<T: Clone> {
    Node {
        val: T,
        ltree: Box<Tree<T>>,
        rtree: Box<Tree<T>>,
    },
    Empty,
}

use self::Tree::*;

impl<T> Set<T> for Tree<T>
where
    T: Clone + PartialOrd,
{
    fn empty() -> Tree<T> {
        Empty
    }

    fn insert(&self, elt: T) -> Tree<T> {
        match self {
            Empty => Node {
                val: elt,
                ltree: Box::new(Empty),
                rtree: Box::new(Empty),
            },
            Node { val, ltree, rtree } => {
                if elt < *val {
                    Node {
                        val: val.clone(),
                        ltree: Box::new(ltree.insert(elt)),
                        rtree: Box::new(*rtree.clone()),
                    }
                } else if elt > *val {
                    Node {
                        val: val.clone(),
                        ltree: Box::new(*ltree.clone()),
                        rtree: Box::new(rtree.insert(elt)),
                    }
                } else {
                    self.clone()
                }
            }
        }
    }

    // Section 2.2 first version

    //    fn member(&self, elt: &T) -> bool {
    //        match self {
    //            Empty => false,
    //            Node { val, ltree, rtree } => {
    //                if elt < val {
    //                    ltree.member(&elt)
    //                } else if elt > val {
    //                    rtree.member(&elt)
    //                } else {
    //                    true
    //                }
    //            }
    //        }
    //    }

    // Exercise 2.2

    fn member(&self, elt: &T) -> bool {
        fn member_impl<T: Clone + PartialOrd>(tree: &Tree<T>, elt: &T, possible: &T) -> bool {
            match tree {
                Empty => elt == possible,
                Node { val, ltree, rtree } => {
                    if elt <= val {
                        member_impl(&ltree, &elt, &val)
                    } else {
                        member_impl(&rtree, &elt, &possible)
                    }
                }
            }
        }
        match self {
            Empty => false,
            Node { val, .. } => member_impl(&self, &elt, &val),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tree_test() {
        let tree: Tree<i32> = Tree::empty().insert(0).insert(1).insert(3);
        assert_eq!(tree.member(&0), true);
        assert_eq!(tree.member(&1), true);
        assert_eq!(tree.member(&2), false);
    }

}
