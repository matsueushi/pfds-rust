trait Stack<T>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(self, _: T) -> Self;
    fn head(&self) -> Option<T>;
    fn tail(self) -> Option<Self>;
}

enum CustomStack<T> {
    Cons(T, Box<CustomStack<T>>),
    Nil,
}

use self::CustomStack::*;

impl<T: Copy> Stack<T> for CustomStack<T> {
    fn empty() -> CustomStack<T> {
        Nil
    }

    fn is_empty(&self) -> bool {
        match self {
            Nil => true,
            _ => false,
        }
    }

    fn cons(self, s: T) -> CustomStack<T> {
        Cons(s, Box::new(self))
    }

    fn head(&self) -> Option<T> {
        match self {
            Nil => None,
            Cons(x, _) => Some(*x),
        }
    }

    fn tail(self) -> Option<CustomStack<T>> {
        match self {
            Nil => None,
            Cons(_, x) => Some(*x),
        }
    }
}

#[test]
fn stack_test() {
    let stack: CustomStack<i32> = CustomStack::empty();
    assert_eq!(stack.is_empty(), true);
    let stack2 = CustomStack::empty().cons(1).cons(2);
    assert_eq!(stack2.is_empty(), false);
    assert_eq!(stack2.head(), Some(2));
    assert_eq!(stack2.tail().unwrap().head(), Some(1));
}
