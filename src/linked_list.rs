#![allow(dead_code)]
/*
 * LinkedList
 * Insert and remove from head/edges O(1)/constant time
 * get, Traversal is O(N) 
 */
use std::rc::Rc;
use std::cell::RefCell;

type Cell<T> = Rc<RefCell<Option<Node<T>>>>;

pub struct LinkedList<T> {
    head: Cell<T>,
    len: usize,
} 

struct Node<T> {
    val: T,
    next: Cell<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: Rc::new(RefCell::new(None)),
            len: 0,
        }
    }

    pub fn add(&mut self, val: T) {
        let n = Node {
            val,
            next: self.head.clone(),
        };
        self.len += 1;
        self.head = Rc::new(RefCell::new(Some(n)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_works() {
        let l = LinkedList::<isize>::new();
        assert_eq!(0, l.len);
    }
}
