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
    fn replace_right_with_left_child(&mut self) -> Option<Box<Self>> {
        if self.right.as_mut().unwrap().left.is_some() {
            let l = self.right.as_mut().unwrap().left.take().unwrap();
            self.right.replace(l)
        } else {
            self.right.take()
        }
    }

    // bad name, not waht it does really
    fn recur_right(&mut self) -> Option<Box<Self>> {
        if let Some(n) = &self.right {
            if n.right.is_some() {
                self.recur_right()
            } else {
                self.replace_right_with_left_child()
            }
        } else {
            None
        }
    }

    // this doesn't work for deleting root node
    // much more complicated search: TODONE
    // actually deleting the value and fixing the tree:
    // Left: TODONE, Right: TODO
    fn delete(&mut self, value: T) -> Option<T> {
        if value < self.value {
            match self.left.as_mut() {
                None => None,
                Some(n) if value == n.value => {
                    // get swap-node from n.left?.right(most) || n.right?
                    let swap_node = if n.left.is_some() {
                        let rightmost = n.left.as_mut().unwrap().recur_right();
                        if rightmost.is_some() {
                            rightmost
                        } else {
                            n.replace_left_with_left_child()
                        }
                    } else if n.right.is_some() {
                        n.replace_right_with_left_child()
                    } else {
                        None
                    };
                    // set self.left.value = swap-node.value
                    if let Some(swap_n) = swap_node {
                        n.value = swap_n.value;
                    } else {
                        self.left = None;
                    }
                    // return deleted value
                    Some(value)
                },
                Some(n) if value < n.value => {
                    n.left.as_mut().and_then(|n| n.delete(value))
                },
                Some(n) => {
                    n.right.as_mut().and_then(|n| n.delete(value))
                },
            }
        } else {
            match self.right.as_mut() {
                None => None,
                Some(n) if value == n.value => Some(value),
                Some(n) if value < n.value => {
                    n.left.as_mut().and_then(|n| n.delete(value))
                },
                Some(n) => {
                    n.right.as_mut().and_then(|n| n.delete(value))
                },
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
        if let Some(node) = self.root.as_mut() {
            node.delete(value)
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
        dbg!(&bst);
        assert!(bst.search(10));
        assert!(bst.search(3458));
        assert!(bst.search(4));
        assert!(!bst.search(101));
        assert!(!bst.search(383));
    }
}
