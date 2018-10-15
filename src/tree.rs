// Section 2.2

trait Set<T> {
    fn empty() -> Self;
    fn insert(&self, elf: T) -> Self;
    fn member(&self, elt: &T) -> bool;
}

use std::rc::Rc;

#[derive(Clone, Debug)]
enum Tree<T: Clone> {
    Node {
        val: T,
        ltree: Rc<Tree<T>>,
        rtree: Rc<Tree<T>>,
    },
    Empty,
}

use self::Tree::*;

impl<T: Clone + PartialOrd> Set<T> for Tree<T> {
    fn empty() -> Tree<T> {
        Empty
    }

    // Section 2.2 first version

    //    fn insert(&self, elt: T) -> Tree<T> {
    //        match self {
    //            Empty => Node {
    //                val: elt,
    //                ltree: Rc::new(Empty),
    //                rtree: Rc::new(Empty),
    //            },
    //            Node { val, ltree, rtree } => {
    //                if elt < *val {
    //                    Node {
    //                        val: val.clone(),
    //                        ltree: Rc::new(ltree.insert(elt)),
    //                        rtree: Rc::clone(&rtree),
    //                    }
    //                } else if elt > *val {
    //                    Node {
    //                        val: val.clone(),
    //                        ltree: Rc::clone(&ltree),
    //                        rtree: Rc::new(rtree.insert(elt)),
    //                    }
    //                } else {
    //                    self.clone()
    //                }
    //            }
    //        }
    //    }

    // Exercise 2.3

    //    fn insert(&self, elt: T) -> Tree<T> {
    //        fn insert_impl<T: Clone + PartialOrd>(tree: &Tree<T>, elt: T) -> Option<Tree<T>> {
    //            match tree {
    //                Empty => Some(Node {
    //                    val: elt,
    //                    ltree: Rc::new(Empty),
    //                    rtree: Rc::new(Empty),
    //                }),
    //                Node { val, ltree, rtree } => {
    //                    if elt < *val {
    //                        insert_impl(&ltree, elt).map(|x| {
    //                            Node {
    //                                val: val.clone(),
    //                                ltree: Rc::new(x),
    //                                rtree: Rc::clone(&rtree),
    //                            }
    //                        })
    //                    } else if elt > *val {
    //                        insert_impl(&rtree, elt).map(|x| {
    //                            Node {
    //                                val: val.clone(),
    //                                ltree: Rc::clone(&ltree),
    //                                rtree: Rc::new(x),
    //                            }
    //                        })
    //                    } else {
    //                        None
    //                    }
    //                }
    //            }
    //        }
    //
    //    match insert_impl(&self, elt) {
    //        Some(x) => x,
    //        None => self.clone(),
    //    }
    //}

    // Exercise 2.4

    fn insert(&self, elt: T) -> Tree<T> {
        fn insert_impl<T: Clone + PartialOrd>(
            tree: &Tree<T>,
            elt: T,
            possible: &T,
        ) -> Option<Tree<T>> {
            match tree {
                Empty => {
                    if elt == *possible {
                        None
                    } else {
                        Some(Node {
                            val: elt,
                            ltree: Rc::new(Empty),
                            rtree: Rc::new(Empty),
                        })
                    }
                }
                Node { val, ltree, rtree } => {
                    if elt <= *val {
                        insert_impl(&ltree, elt, &val).map(|x| {
                            Node {
                                val: val.clone(),
                                ltree: Rc::new(x),
                                rtree: Rc::clone(&rtree),
                            }
                        })
                    } else {
                        insert_impl(&rtree, elt, &possible).map(|x| {
                            Node {
                                val: val.clone(),
                                ltree: Rc::clone(&ltree),
                                rtree: Rc::new(x),
                            }
                        })
                    }
                }
            }
        }

        match self {
            Empty => Node {
                val: elt,
                ltree: Rc::new(Empty),
                rtree: Rc::new(Empty),
            },
            Node { val, .. } => {
                match insert_impl(&self, elt, &val) {
                    Some(x) => x,
                    None => self.clone(),
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

// Exercise 2.5 (a)
fn complete<T: Clone + PartialOrd>(elt: T, len: u32) -> Tree<T> {
    match len {
        0 => Empty,
        _ => {
            let subtree = Rc::new(complete(elt.clone(), len - 1));
            Node {
                val: elt,
                ltree: Rc::clone(&subtree),
                rtree: Rc::clone(&subtree),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tree_test() {
        let tree = Tree::empty().insert(1).insert(3).insert(0).insert(1);
        println!("{:?}", &tree);
        assert_eq!(tree.member(&0), true);
        assert_eq!(tree.member(&1), true);
        assert_eq!(tree.member(&2), false);
        assert_eq!(tree.member(&3), true);
    }

    #[test]
    fn complete_test() {
        let tree = complete(1, 2);
        println!("{:?}", &tree);
    }
}
