// Section 3.1

use std::rc::Rc;

trait Heap<T: Clone + PartialOrd>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn insert(&self, elt: T) -> Self;
    fn merge(&self, other: &Self) -> Self;

    fn find_min(&self) -> Option<&T>;
    fn delete_min(&self) -> Option<Self>;
}

#[derive(Clone, Debug)]
pub enum LeftistHeap<T: Clone> {
    Node {
        rank: u32,
        val: T,
        ltree: Rc<LeftistHeap<T>>,
        rtree: Rc<LeftistHeap<T>>,
    },
    Empty,
}

use self::LeftistHeap::*;

impl<T: Clone> LeftistHeap<T> {
    fn rank(&self) -> u32 {
        match self {
            Empty => 0,
            Node { rank, .. } => *rank,
        }
    }

    fn singleton(elt: T) -> Self {
        Node {
            rank: 1,
            val: elt,
            ltree: Rc::new(Empty),
            rtree: Rc::new(Empty),
        }
    }
}

impl<T: Clone + PartialOrd> Heap<T> for LeftistHeap<T> {
    fn empty() -> Self {
        Empty
    }

    fn is_empty(&self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
    }

    fn merge(&self, other: &LeftistHeap<T>) -> Self {
        fn make_tree<T: Clone + PartialOrd>(
            val: T,
            ltree: Rc<LeftistHeap<T>>,
            rtree: Rc<LeftistHeap<T>>,
        ) -> LeftistHeap<T> {
            if ltree.rank() >= rtree.rank() {
                Node {
                    rank: rtree.rank() + 1,
                    val: val,
                    ltree: ltree,
                    rtree: rtree,
                }
            } else {
                Node {
                    rank: ltree.rank() + 1,
                    val: val,
                    ltree: rtree,
                    rtree: ltree,
                }
            }
        }

        match self {
            Empty => other.clone(),
            Node {
                rank: _,
                val: x,
                ltree: ltree1,
                rtree: rtree1,
            } => match other {
                Empty => self.clone(),
                Node {
                    rank: _,
                    val: y,
                    ltree: ltree2,
                    rtree: rtree2,
                } => {
                    if x <= y {
                        make_tree(x.clone(), ltree1.clone(), Rc::new(rtree1.merge(other)))
                    } else {
                        make_tree(y.clone(), ltree2.clone(), Rc::new(self.merge(rtree2)))
                    }
                }
            },
        }
    }

    // Section 3.1 original version

    // fn insert(&self, elt: T) -> Self {
    //     Self::singleton(elt).merge(self)
    // }

    // Section 3.2

    fn insert(&self, elt: T) -> Self {
        match self {
            Empty => Self::singleton(elt),
            Node {
                rank: _,
                val,
                ltree,
                rtree,
            } => {
                let (t, b) = match elt <= *val {
                    true => (&elt, val),
                    false => (val, &elt),
                };
                let (lt, rt) = match **rtree {
                    Empty => (*ltree, Self::singleton(*b)),
                    Node {
                        rank: _, val: y2, ..
                    } => {
                        if let Node {
                            rank: _, val: y1, ..
                        } = self
                        {
                            if y1 <= y2 {
                                (*ltree, Rc::new(rtree.insert(b)))
                            } else {
                                (Rc::new(ltree.insert(b)), *rtree)
                            }
                        }
                    }
                };
                if lt.rank() >= rt.rank() {
                    Node {
                        rank: rt.rank() + 1,
                        val: *t,
                        ltree: lt,
                        rtree: rt,
                    }
                } else {
                    Node {
                        rank: lt.rank() + 1,
                        val: *t,
                        ltree: rt,
                        rtree: lt,
                    }
                }
            }
        }
    }

    fn find_min(&self) -> Option<&T> {
        match self {
            Empty => None,
            Node { rank: _, val, .. } => Some(&val),
        }
    }

    fn delete_min(&self) -> Option<LeftistHeap<T>> {
        match self {
            Empty => None,
            Node {
                rank: _,
                val: _,
                ltree,
                rtree,
            } => Some(ltree.merge(&**rtree)),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn heap_test() {
        let empty_heap: LeftistHeap<i32> = LeftistHeap::empty();
        assert!(empty_heap.is_empty());

        let heap = LeftistHeap::empty().insert(2).insert(1).insert(0);
        println!("heap: {:?}", &heap);

        let heap2 = LeftistHeap::empty().insert(5).insert(1).insert(3);
        let heap3 = heap.merge(&heap2);
        println!("heap3: {:?}", &heap3);

        assert_eq!(heap3.find_min(), Some(&0));

        println!("delete_min: {:?}", heap3.delete_min());
    }
}
