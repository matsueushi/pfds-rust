// 2.1 List

trait Stack<T>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn cons(&self, elt: T) -> Self;
    fn head(&self) -> Option<&T>;
    fn tail(&self) -> Option<Self>;
}

#[derive(Clone)]
enum StackNode<T: Clone> {
    Cons(T, Box<StackNode<T>>),
    Nil,
}

struct CustomStack<T: Clone> {
    node: StackNode<T>,
}

use self::StackNode::*;

impl<T> Stack<T> for CustomStack<T>
where
    T: Clone,
{
    fn empty() -> CustomStack<T> {
        CustomStack { node: Nil }
    }

    fn is_empty(&self) -> bool {
        match self.node {
            Nil => true,
            _ => false,
        }
    }

    fn cons(&self, elt: T) -> CustomStack<T> {
        CustomStack { node: Cons(elt, Box::new(self.node.clone())) }
    }

    fn head(&self) -> Option<&T> {
        match self.node {
            Nil => None,
            Cons(ref x, _) => Some(x),
        }
    }

    fn tail(&self) -> Option<CustomStack<T>> {
        match &self.node {
            Nil => None,
            Cons(_, x) => Some(CustomStack { node: *x.clone() }),
        }
    }
}

use std::collections::LinkedList;

struct LinkedListStack<T: Clone> {
    node: LinkedList<T>,
}

impl<T> Stack<T> for LinkedListStack<T>
where
    T: Clone,
{
    fn empty() -> LinkedListStack<T> {
        LinkedListStack { node: LinkedList::new() }
    }

    fn is_empty(&self) -> bool {
        self.node.is_empty()
    }

    fn cons(&self, elf: T) -> LinkedListStack<T> {
        let mut copy = self.node.clone();
        copy.push_front(elf);
        LinkedListStack { node: copy }
    }

    fn head(&self) -> Option<&T> {
        self.node.front()
    }

    fn tail(&self) -> Option<LinkedListStack<T>> {
        let mut copy = self.node.clone();
        match copy.pop_front() {
            None => None,
            Some(_) => Some(LinkedListStack { node: copy }),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn custom_stack_test() {
        let stack: CustomStack<i32> = CustomStack::empty();
        assert_eq!(stack.is_empty(), true);
        let stack2 = CustomStack::empty().cons(1).cons(2);
        assert_eq!(stack2.is_empty(), false);
        assert_eq!(stack2.head(), Some(&2));
        assert_eq!(stack2.tail().unwrap().head(), Some(&1));
    }

    #[test]
    fn linked_list_stack_test() {
        let stack: LinkedListStack<i32> = LinkedListStack::empty();
        assert_eq!(stack.is_empty(), true);
        let stack2 = LinkedListStack::empty().cons(1).cons(2);
        assert_eq!(stack2.is_empty(), false);
        assert_eq!(stack2.head(), Some(&2));
        assert_eq!(stack2.tail().unwrap().head(), Some(&1));
    }
}
