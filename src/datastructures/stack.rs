use std::mem;

pub struct Stack<T>{
    current: Link<T>,
    length: usize
}

enum Link<T>{
    Empty,
    More(Box<Item<T>>)
}

struct Item<T>{
    value: T,
    next: Link<T>
}

impl<T> Stack<T>{
    pub fn new() -> Stack<T>{
        Stack {
            current: Link::Empty,
            length: 0
        }
    }

    pub fn push(&mut self, item: T){
        let new_item = Box::new(Item {
            value: item,
            next: mem::replace(&mut self.current, Link::Empty)
        });

        self.current = Link::More(new_item);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.length == 0 {
            panic!("Stack empty");
        }
        match mem::replace(&mut self.current, Link::Empty){
            Link::Empty => None,
            Link::More(node) => {
                self.current = node.next;
                self.length -= 1;
                Some(node.value)
            }
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests{
    use crate::datastructures::stack::Stack;

    #[test]
    fn push_test(){
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(3, stack.pop().unwrap());
        assert_eq!(2, stack.pop().unwrap());
        assert_eq!(1, stack.pop().unwrap());
    }

    #[test]
    #[should_panic]
    fn push_panic_test(){
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.pop();
        stack.pop();
    }
}