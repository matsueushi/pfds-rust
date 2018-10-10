enum Stack<T> {
    Cons(T, Box<Stack<T>>),
    Nil,
}

use self::Stack::*;

impl<T: Copy> Stack<T> {
    fn empty() -> Stack<T> {
        Nil
    }

    fn is_empty(&self) -> bool {
        match self {
            Nil => true,
            _ => false,
        }
    }

    fn cons(self, s: T) -> Stack<T> {
        Cons(s, Box::new(self))
    }

    fn head(&self) -> Option<T> {
        match self {
            Nil => None,
            Cons(x, _) => Some(*x),
        }
    }

    fn tail(self) -> Option<Stack<T>> {
        match self {
            Nil => None,
            Cons(_, x) => Some(*x),
        }
    }
}

#[test]
fn stack_test() {
    let stack: Stack<i32> = Stack::empty();
    assert_eq!(stack.is_empty(), true);
    let stack2 = Stack::empty().cons(1).cons(2);
    assert_eq!(stack2.is_empty(), false);
    assert_eq!(stack2.head(), Some(2));
    assert_eq!(stack2.tail().unwrap().head(), Some(1));
}
