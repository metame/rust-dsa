#![allow(dead_code)]
/*
 * LinkedList
 * Insert and remove from head/edges O(1)/constant time
 * get, Traversal is O(N)
 * new
 * add
 * get
 * remove
 */
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;

type Cell<T> = Rc<RefCell<Node<T>>>;

pub struct LinkedList<T> {
    head: Option<Cell<T>>,
    len: usize,
}

struct Node<T> {
    val: T,
    next: Option<Cell<T>>,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    pub fn add(&mut self, val: T) {
        let prev_head = if let Some(cell) = &self.head {
            Some(Rc::clone(&cell))
        } else {
            None
        };
        let n = Node {
            val,
            next: prev_head,
        };
        self.len += 1;
        self.head = Some(Rc::new(RefCell::new(n)));
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        let mut cell = self.head.clone().unwrap();
        for _ in 1..=index {
            let next_cell = cell.borrow().deref().next.clone().unwrap();
            cell = next_cell;
        }
        cell.borrow();
        let val = Ref::clone(&cell.borrow()).deref().val.clone();
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_works() {
        let mut l = LinkedList::<isize>::new();
        assert_eq!(0, l.len);
        l.add(200);
        l.add(-2);
        l.add(4123);
        assert_eq!(3, l.len);
        assert_eq!(Some(200), l.get(2));
        assert_eq!(Some(-2), l.get(1));
        assert_eq!(Some(4123), l.get(0));
    }
}
