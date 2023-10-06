#![allow(dead_code)]
/* Queue:
 * FiFo - e.g. SQS, RabbitMQ
 * queue - adds to tail
 * deque - removes from head
 * peek - looks at next value in queue without consuming
 */
use std::fmt::Debug;

use crate::doubly_linked_list::DubLinkedList;

pub struct Queue<T> {
    pub size: usize,
    list: DubLinkedList<T>,
}

impl<T: Debug> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            size: 0,
            list: DubLinkedList::<T>::new(),
        }
    }

    pub fn queue(&mut self, item: T) {
        self.list.push_back(item);
        self.size += 1;
    }

    pub fn deque(&mut self) -> Option<T> {
        if self.size != 0 {
            self.size -= 1;
        }
        self.list.pop_front()
    }

    pub fn peek(&self) -> Option<T> where T: Clone {
        self.list.get(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_works() {
        let mut q = Queue::<isize>::new();
        q.deque();
        assert_eq!(0, q.size);
        assert_eq!(None, q.peek());
        q.queue(-34);
        assert_eq!(1, q.size);
        assert_eq!(Some(-34), q.peek());
        q.queue(45);
        q.queue(3049);
        assert_eq!(3, q.size);
        assert_eq!(Some(-34), q.deque());
        assert_eq!(Some(45), q.peek());
        assert_eq!(Some(45), q.deque());
        assert_eq!(Some(3049), q.peek());
        assert_eq!(Some(3049), q.deque());
        assert_eq!(0, q.size);
        assert_eq!(None, q.deque());
        assert_eq!(0, q.size);
    }
}
