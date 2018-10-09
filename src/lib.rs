enum CustomStack {
    Cons(i32, Box<CustomStack>),
    Nil,
}

struct Stack {
    head: CustomStack,
}

use self::CustomStack::*;

impl Stack {
    fn empty() -> Stack {
        Stack { head: Nil }
    }

    fn is_empty(&self) -> bool {
        match self.head {
            Nil => true,
            _ => false,
        }
    }

    fn cons(&mut self, s: i32) -> &mut Stack {
        self.head = Cons(s, Box::new(::std::mem::replace(&mut self.head, Nil)));
        self
    }

    fn head(&self) -> Option<i32> {
        match self.head {
            Nil => None,
            Cons(x, _) => Some(x),
        }
    }
}

#[test]
fn stack_test() {
    let mut stack = Stack::empty();
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.cons(32).head(), Some(32));
}
