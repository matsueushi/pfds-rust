trait Stack<T>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(&mut self, _: T) -> &mut Self;
    fn head(&self) -> Option<&T>;
    fn tail(&mut self) -> Option<&mut Self>;
}

enum StackNode<T> {
    Cons(T, Box<StackNode<T>>),
    Nil,
}

struct CustomStack<T> {
    node: StackNode<T>,
}

use self::StackNode::*;

impl<T: Copy> Stack<T> for CustomStack<T> {
    fn empty() -> CustomStack<T> {
        CustomStack { node: Nil }
    }

    fn is_empty(&self) -> bool {
        match self.node {
            Nil => true,
            _ => false,
        }
    }

    fn cons(&mut self, elt: T) -> &mut CustomStack<T> {
        self.node = Cons(elt, Box::new(::std::mem::replace(&mut self.node, Nil)));
        self
    }
    fn head(&self) -> Option<&T> {
        match self.node {
            Nil => None,
            Cons(ref x, _) => Some(x),
        }
    }

    fn tail(&mut self) -> Option<&mut CustomStack<T>> {
        match &mut self.node {
            Nil => None,
            Cons(_, x) => {
                self.node = ::std::mem::replace(x, Nil);
                Some(self)
            }
        }
    }
}

use std::collections::LinkedList;

struct LinkedListStack<T> {
    node: LinkedList<T>,
}

impl<T: Copy> Stack<T> for LinkedListStack<T> {
    fn empty() -> LinkedListStack<T> {
        LinkedListStack { node: LinkedList::new() }
    }

    fn is_empty(&self) -> bool {
        self.node.is_empty()
    }

    fn cons(&mut self, elf: T) -> &mut LinkedListStack<T> {
        self.node.push_front(elf);
        self
    }

    fn head(&self) -> Option<&T> {
        self.node.front()
    }

    fn tail(&mut self) -> Option<&mut LinkedListStack<T>> {
        match self.node.pop_front() {
            None => None,
            Some(_) => Some(self),
        }
    }
}

#[test]
fn custom_stack_test() {
    let stack: CustomStack<i32> = CustomStack::empty();
    assert_eq!(stack.is_empty(), true);
    let mut stack2 = CustomStack::empty();
    let stack2 = stack2.cons(1).cons(2);
    assert_eq!(stack2.is_empty(), false);
    assert_eq!(stack2.head(), Some(&2));
    assert_eq!(stack2.tail().unwrap().head(), Some(&1));
}

#[test]
fn linked_list_stack_test() {
    let stack: LinkedListStack<i32> = LinkedListStack::empty();
    assert_eq!(stack.is_empty(), true);
    let mut stack2 = LinkedListStack::empty();
    let stack2 = stack2.cons(1).cons(2);
    assert_eq!(stack2.is_empty(), false);
    assert_eq!(stack2.head(), Some(&2));
    assert_eq!(stack2.tail().unwrap().head(), Some(&1));
}
