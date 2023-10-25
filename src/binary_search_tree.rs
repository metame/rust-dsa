#![allow(dead_code)]
use std::fmt::Debug;

/**
 * Binary Search Tree
 * sorted
 * Node.left.value < Node.value
 * Node.right.value >= Node.value
 * search, insertion & removal: O(log N)
 * Vec: input & output, impl From<Vec<T>>, Into<Vec<T>>,
*/
#[derive(Clone, Debug)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
    fn insert_child(&mut self, node: Node<T>) {
        let branch = if node.value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };
        if branch.is_none() {
            branch.replace(Box::new(node));
        } else {
            branch.as_mut().unwrap().insert_child(node);
        }
    }

    /// Assumes that self.list.is_some()
    fn replace_left_with_left_child(&mut self) -> Option<Box<Self>> {
        if self.left.as_mut().unwrap().left.is_some() {
            let l = self.left.as_mut().unwrap().left.take().unwrap();
            self.left.replace(l)
        } else {
            self.left.take()
        }
    }

    /// Assumes that self.right.is_some()
    fn replace_right_with_child(&mut self) -> Option<Box<Self>> {
        if let Some(left_child) = self.right.as_mut().unwrap().left.take() {
            self.right.replace(left_child)
        } else if let Some(right_child) = self.right.as_mut().unwrap().right.take() {
            self.right.replace(right_child)
        } else {
            self.right.take()
        }
    }

    /// bad name, not what it does really
    fn recur_right(&mut self) -> Option<Box<Self>> {
        if let Some(n) = &mut self.right {
            if n.right.is_some() {
                n.recur_right()
            } else {
                self.replace_right_with_child()
            }
        } else {
            None
        }
    }

    fn leftmost<'a>(&'a self, stack: &mut Vec<&'a Node<T>>) -> &Self {
        if let Some(n) = &self.left {
            stack.push(self);
            n.leftmost(stack)
        } else {
            self
        }
    }

    fn swap_node(&mut self) -> Option<&mut Self> {
        let swap_node = if let Some(left) = &mut self.left {
            let rightmost = left.recur_right();
            if rightmost.is_some() {
                rightmost
            } else {
                self.replace_left_with_left_child()
            }
        } else if self.right.is_some() {
            self.replace_right_with_child()
        } else {
            None
        };

        if let Some(swap_n) = swap_node {
            self.value = swap_n.value;
            Some(self)
        } else {
            None
        }
    }

    // this doesn't work for deleting root node
    fn delete_from(&mut self, value: T) -> Option<T> {
        if value < self.value {
            match self.left.as_mut() {
                None => None,
                Some(n) if value == n.value => {
                    if n.swap_node().is_none() {
                        self.left = None;
                    }
                    Some(value)
                },
                Some(n) => n.delete_from(value),
            }
        } else {
            match self.right.as_mut() {
                None => None,
                Some(n) if value == n.value => {
                    if n.swap_node().is_none() {
                        self.right = None;
                    }
                    Some(value)
                }
                Some(n) => n.delete_from(value),
            }
        }
    }

    fn search(&self, value: T) -> Option<&Self> {
        if value == self.value {
            Some(self)
        } else if value < self.value {
            self.left.as_ref().and_then(|n| n.search(value))
        } else {
            self.right.as_ref().and_then(|n| n.search(value))
        }
    }

    /// In order traversal applying f to self.value, consuming self
    fn traverse<F>(mut self, f: &mut F)
    where F: FnMut(T),
    {
        if let Some(l) = self.left.take() {
            l.traverse(f);
        }
        let right = self.right.take();
        f(self.value);

        if let Some(r) = right {
            r.traverse(f);
        }
    }

}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let node = Node {
            value,
            left: None,
            right: None,
        };

        if self.root.is_none() {
            self.root = Some(Box::new(node));
        } else {
            self.root.as_mut().unwrap().insert_child(node);
        }
    }

    pub fn search(&self, value: T) -> bool {
        self.root.as_ref().and_then(|n| n.search(value)).is_some()
    }

    // if deleted node doesn't have children, remove from parent and return value
    // if it has children, take the node.left.right(most) node and replace it
    pub fn delete(&mut self, value: T) -> Option<T> {
        match self.root.as_mut() {
            None => None,
            Some(n) if value == n.value => {
                if n.swap_node().is_none() {
                    self.root = None;
                }
                Some(value)
            },
            Some(n) => n.delete_from(value),
        }
    }

    /// In order traversal applying `f` to each `T` consuming `self`
    pub fn traverse<F>(mut self, f: &mut F)
    where F: FnMut(T),
    {
        if let Some(n) = self.root.take() {
            n.traverse(f);
        }
    }

    pub fn iter<'a>(&'a self) -> BSTIterator<'a, T> {
        let mut stack = Vec::new();
        let current = self.root.as_ref().map(|n| n.leftmost(&mut stack));
        BSTIterator {
            stack,
            root: self.root.as_ref(),
            current,
        }
    }
}

pub struct BSTIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
    root: Option<&'a Box<Node<T>>>,
    current: Option<&'a Node<T>>,
}

impl<'a, T: PartialOrd> Iterator for BSTIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.current {
            let v = &n.value;
            // check if right, then right.leftmost, else right
            if let Some(r) = n.right.as_ref() {
                self.current = Some(r.leftmost(&mut self.stack));
            } else {
                // go up stack
                self.current = self.stack.pop();
            }
            Some(v)
        } else {
            None
        }
    }
}

impl<T: PartialOrd> From<BinarySearchTree<T>> for Vec<T> {
    fn from(bst: BinarySearchTree<T>) -> Self {
        let mut vec = Vec::new();
        bst.traverse(&mut |v: T| vec.push(v));
        vec
    }
}

impl<T: PartialOrd + Debug> From<Vec<T>> for BinarySearchTree<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut bst = BinarySearchTree::<T>::new();
        for v in vec {
            bst.insert(v);
        }
        bst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_works() {
        let mut bst = BinarySearchTree::<usize>::new();
        bst.insert(9);
        bst.insert(5);
        bst.insert(4);
        bst.insert(1);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);
        bst.insert(17);
        bst.insert(900);
        bst.insert(800);
        let v: Vec<&usize> = bst.iter().collect();
        dbg!(&bst);
        let expected = vec![1, 4, 4, 5, 6, 8, 9, 17, 800, 900];
        let expected: Vec<&usize> = expected.iter().collect();
        assert_eq!(expected, v);
    }

    #[test]
    fn traverse_works() {
        let mut bst = BinarySearchTree::<usize>::new();
        bst.insert(9);
        bst.insert(5);
        bst.insert(4);
        bst.insert(1);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);
        bst.insert(17);
        bst.insert(900);
        bst.insert(800);
        assert_eq!(vec![1, 4, 4, 5, 6, 8, 9, 17, 800, 900], Vec::from(bst));
    }

    #[test]
    fn from_vec_works() {
        let vec = vec![3, 10, 0];
        let bst = BinarySearchTree::from(vec);
        dbg!(&bst);
        assert!(bst.search(10));
        assert!(bst.search(3));
        assert!(bst.search(0));
    }

    #[test]
    fn bst_works() {
        let mut bst = BinarySearchTree::<usize>::new();
        bst.insert(10);
        bst.insert(7);
        bst.insert(15);
        bst.insert(9);
        bst.insert(4);
        bst.insert(17);
        bst.insert(3458);
        bst.insert(2000);
        assert!(bst.search(10));
        assert!(bst.search(3458));
        assert!(bst.search(4));
        assert!(!bst.search(101));
        assert!(!bst.search(383));
        bst.insert(1);
        bst.insert(5);
        bst.insert(6);
        assert_eq!(Some(7), bst.delete(7));
        bst.insert(4);
        assert_eq!(Some(6), bst.delete(6));
        dbg!(&bst);
        assert_eq!(Some(3458), bst.delete(3458));
        dbg!(&bst);
        assert_eq!(Some(15), bst.delete(15));
        dbg!(&bst);
        bst.insert(1000);
        bst.insert(900);
        bst.insert(800);
        assert_eq!(Some(1000), bst.delete(1000));
        dbg!(&bst);
        assert_eq!(Some(2000), bst.delete(2000));
        dbg!(&bst);
        assert_eq!(Some(10), bst.delete(10));
        dbg!(&bst);
    }
}
