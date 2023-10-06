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
use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct DubLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    len: usize,
}

impl<T: fmt::Debug> fmt::Debug for DubLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DubLL {{ tail: {:?}, len: {:?} }}", self.tail, self.len)
    }
}

#[derive(Clone)]
struct Node<T> {
    val: T,
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ val: {:?}, prev: {:?}}}", self.val, self.prev)
    }
}

impl<T: fmt::Debug> DubLinkedList<T> {
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

    pub fn push_back(&mut self, val: T) {
        if self.tail.is_none() {
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
            let old_tail = self.tail.take().unwrap();
            let n = Node {
                val,
                prev: Some(Rc::clone(&old_tail)),
                next: None,
            };
            let link = Rc::new(RefCell::new(n));
            old_tail.borrow_mut().next = Some(Rc::clone(&link));
            self.tail = Some(link);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let head = self.head.take().unwrap();
        let new_head = if let Some(link) = Rc::clone(&head).borrow_mut().next.take() {
            link.borrow_mut().prev = None;
            Some(link)
        } else {
            None
        };

        self.len -= 1;
        self.head = new_head;
        if self.len == 0 {
            self.tail = None;
        }

        let n = match Rc::try_unwrap(head) {
            Ok(node) => node,
            Err(_) => panic!("we did bad thing in pop_front, how dare you!"),
        };
        let v = n.into_inner().val;
        Some(v)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let tail = self.tail.take().unwrap();
        let new_tail = if let Some(link) = Rc::clone(&tail).borrow_mut().prev.take() {
            link.borrow_mut().next = None;
            Some(link)
        } else {
            None
        };

        self.len -= 1;
        self.tail = new_tail;
        if self.len == 0 {
            self.head = None;
        }

        let n = match Rc::try_unwrap(tail) {
            Ok(node) => node,
            Err(_) => panic!("we did a bad thing again in pop_back, wtaf!"),
        };
        let v = n.into_inner().val;
        Some(v)
    }

    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        if index >= self.len {
            return None;
        }
        let mut node = self.head.as_ref().unwrap().borrow().deref().clone();
        for _ in 1..=index {
            let n = node.next.as_ref();
            let o = n.unwrap();
            let next_node = o.borrow().deref().clone();
            node = next_node;
        }
        Some(node.val.clone())
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
        assert_eq!(Some(200), l.get(2));
        assert_eq!(Some(-2), l.get(1));
        assert_eq!(Some(4123), l.get(0));
        assert_eq!(Some(4123), l.pop_front());
        assert_eq!(Some(-2), l.pop_front());
        assert_eq!(Some(200), l.pop_front());
        assert_eq!(None, l.pop_front());
        l.push_back(5000);
        l.push_front(-200);
        l.push_back(2);
        assert_eq!(3, l.len);
        // -200, 5000, 2
        assert_eq!(Some(-200), l.get(0));
        assert_eq!(Some(5000), l.get(1));
        assert_eq!(Some(2), l.get(2));
        assert_eq!(Some(2), l.pop_back());
        assert_eq!(Some(5000), l.pop_back());
        assert_eq!(Some(-200), l.pop_back());
        assert_eq!(None, l.pop_back());
        l.pop_back();
        assert_eq!(0, l.len);
    }
}
