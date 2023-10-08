#![allow(dead_code)]
/*
 * LinkedList
 * Insert and remove from head/edges O(1)/constant time
 * get, Traversal is O(N)
 * new
 * push_front
 * get
 * pop_front
 */
use std::boxed::Box;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

// unsafe impl<T: Send> Send for LinkedList<T> {}
// unsafe impl<T: Sync> Sync for LinkedList<T> {}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    pub fn push_front(&mut self, val: T) {
        let prev_head = if let Some(node) = self.head.take() {
            Some(node)
        } else {
            None
        };
        let n = Node {
            val,
            next: prev_head,
        };
        self.len += 1;
        self.head = Some(Box::new(n));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let mut head = self.head.take().unwrap();
        let new_head = if let Some(node) = head.next.take() {
            Some(node)
        } else {
            None
        };

        self.len -= 1;
        self.head = new_head;
        Some(head.val)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let mut cell = self.head.as_ref().unwrap();
        for _ in 1..=index {
            let next_cell = cell.next.as_ref().unwrap();
            cell = next_cell;
        }
        Some(&cell.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_works() {
        let mut l = LinkedList::<isize>::new();
        assert_eq!(0, l.len);
        assert_eq!(None, l.pop_front());
        l.push_front(200);
        l.push_front(-2);
        l.push_front(4123);
        assert_eq!(3, l.len);
        assert_eq!(Some(&200), l.get(2));
        assert_eq!(Some(&-2), l.get(1));
        assert_eq!(Some(&4123), l.get(0));
        assert_eq!(Some(4123), l.pop_front());
        assert_eq!(Some(-2), l.pop_front());
        assert_eq!(Some(200), l.pop_front());
        assert_eq!(None, l.pop_front());
    }
}
