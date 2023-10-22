#![allow(dead_code)]
/**
 * Binary Search Tree
 * sorted
 * Node.left.value < Node.value
 * Node.right.value >= Node.value
 * search, insertion & removal: O(log N)
*/

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + std::fmt::Debug> Node<T> {
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

    // bad name, not waht it does really
    fn recur_right(&mut self) -> Option<Box<Self>> {
        if let Some(n) = &mut self.right {
            dbg!(&n.value);
            if n.right.is_some() {
                n.recur_right()
            } else {
                self.replace_right_with_child()
            }
        } else {
            None
        }
    }

    fn swap_node(&mut self) -> Option<&mut Self> {
        let swap_node = if let Some(left) = &mut self.left {
            let rightmost = left.recur_right();
            if rightmost.is_some() {
                dbg!(&rightmost.as_ref().unwrap().value);
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
        dbg!(&self.value);
        if value < self.value {
            match self.left.as_mut() {
                None => None,
                Some(n) if value == n.value => {
                    if n.swap_node().is_none() {
                        self.left = None;
                    }
                    Some(value)
                },
                Some(n) if value < n.value => n.left.as_mut().and_then(|n| n.delete_from(value)),
                Some(n) => n.right.as_mut().and_then(|n| n.delete_from(value)),
            }
        } else {
            dbg!(self.right.as_ref().map(|n| &n.value));
            match self.right.as_mut() {
                None => None,
                Some(n) if value == n.value => {
                    if n.swap_node().is_none() {
                        self.right = None;
                    }
                    Some(value)
                }
                Some(n) if value < n.value => n.left.as_mut().and_then(|n| n.delete_from(value)),
                Some(n) => n.right.as_mut().and_then(|n| n.delete_from(value)),
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
}

impl<T: PartialOrd + std::fmt::Debug> BinarySearchTree<T> {
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
        if let Some(node) = self.root.as_mut() {
            node.delete_from(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // this isn't working for some reason even though the tree looks right before this
        assert_eq!(Some(2000), bst.delete(2000));

    }
}
