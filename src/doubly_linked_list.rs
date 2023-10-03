#![allow(dead_code)]
/*
 * DubLinkedList
 * Insert and remove from edges O(1)/constant time
 * get, Traversal is O(N)
 * new
 * push_front
 * push_back
 * pop_front
 * pop_back
 * get
 */
use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct DubLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    len: usize,
}

struct Node<T> {
    val: T,
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
}

impl<T> DubLinkedList<T> {
    pub fn new() -> DubLinkedList<T> {
        DubLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, val: T) {
        if self.head.is_none() {
            let n = Node {
                val,
                prev: None,
                next: None,
            };
            let link = Rc::new(RefCell::new(n));
            self.tail = Some(Rc::clone(&link));
            self.head = Some(link);
            self.len += 1;
        } else {
            let old_head = self.head.take().unwrap();
            let n = Node {
                val,
                prev: None,
                next: Some(Rc::clone(&old_head)),
            };
            let link = Rc::new(RefCell::new(n));
            old_head.borrow_mut().prev = Some(Rc::clone(&link));
            self.head = Some(link);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let head = self.head.take().unwrap();
        let new_head = if let Some(link) = Rc::clone(&head).borrow_mut().next.take() {
            Some(link)
        } else {
            None
        };

        self.len -= 1;
        self.head = new_head;
        let n = match Rc::try_unwrap(head) {
            Ok(node) => node,
            Err(_) => panic!("we did bad thing in pop_front, how dare you!"),
        };
        let v = n.into_inner().val;
        Some(v)
    }

    pub fn get(&self, index: usize) -> Option<T> where T: Clone {
        if index >= self.len {
            return None;
        }
        let mut link = self.head.as_ref().unwrap();
        for _ in 1..=index {
            let r = Rc::clone(&link);
            let b = r.borrow();
            let n = b.next.as_ref();
            let o = n.unwrap();
            let next_link = o;
            link = next_link;
        }
        Some(link.borrow().val.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dub_linked_list_works() {
        let mut l = DubLinkedList::<isize>::new();
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
