enum Stack {
    Cons(i32, Box<Stack>),
    Nil,
}

use self::Stack::*;

impl Stack {
    fn empty() -> Stack {
        Nil
    }

    fn is_empty(&self) -> bool {
        match self {
            Nil => true,
            _ => false,
        }
    }

    fn cons(self, s: i32) -> Stack {
        Cons(s, Box::new(self))
    }

    fn head(&self) -> Option<i32> {
        match self {
            Nil => None,
            Cons(x, _) => Some(*x),
        }
    }

    fn tail(self) -> Option<Stack> {
        match self {
            Nil => None,
            Cons(_, x) => Some(*x),
        }
    }
}

#[test]
fn stack_test() {
    let stack = Stack::empty();
    assert_eq!(stack.is_empty(), true);
    let stack2 = Stack::empty().cons(1).cons(2);
    assert_eq!(stack2.is_empty(), false);
    assert_eq!(stack2.head(), Some(2));
    assert_eq!(stack2.tail().unwrap().head(), Some(1));
}
