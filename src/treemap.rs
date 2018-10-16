use tree;

trait FiniteMap<S, T> {
    fn empty() -> Self;
    fn bind(&self, key: S, val: T) -> Self;
    fn lookup(&self, key: S) -> Option<&T>;
}

type TreeMap<S, T> = tree::Tree<(S, T)>;
