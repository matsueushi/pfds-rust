// Excercise 2.6

use super::tree::Tree::*;
use std::rc::Rc;

trait FiniteMap<S, T> {
    fn empty() -> Self;
    fn bind(&self, key: S, val: T) -> Self;
    fn lookup(&self, key: &S) -> Option<&T>;
}

type TreeMap<S, T> = super::tree::Tree<(S, T)>;

impl<S, T> FiniteMap<S, T> for TreeMap<S, T>
where
    S: Clone + PartialOrd,
    T: Clone,
{
    fn empty() -> TreeMap<S, T> {
        Empty
    }

    fn bind(&self, key: S, val: T) -> TreeMap<S, T> {
        match self {
            Empty => Node {
                val: (key, val),
                ltree: Rc::new(Empty),
                rtree: Rc::new(Empty),
            },
            Node {
                val: (k, v),
                ltree,
                rtree,
            } => {
                if key < *k {
                    Node {
                        val: (k.clone(), v.clone()),
                        ltree: Rc::new(ltree.bind(key, val)),
                        rtree: Rc::clone(&rtree),
                    }
                } else if key > *k {
                    Node {
                        val: (k.clone(), v.clone()),
                        ltree: Rc::clone(&ltree),
                        rtree: Rc::new(rtree.bind(key, val)),
                    }
                } else {
                    self.clone()
                }
            }
        }
    }

    fn lookup(&self, key: &S) -> Option<&T> {
        match self {
            Empty => None,
            Node {
                val: (k, v),
                ltree,
                rtree,
            } => {
                if key < k {
                    ltree.lookup(&key)
                } else if key > k {
                    rtree.lookup(&key)
                } else {
                    Some(&v)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn treemap_test() {
        let empty_map: TreeMap<i32, i32> = TreeMap::empty();
        assert_eq!(empty_map.lookup(&42), None);

        let map = TreeMap::empty().bind("abcde", 12345).bind("fghik", 67890);
        assert_eq!(map.lookup(&"abcde"), Some(&12345));
        assert_eq!(map.lookup(&"fghik"), Some(&67890));
        assert_eq!(map.lookup(&"ooooo"), None);
    }
}
