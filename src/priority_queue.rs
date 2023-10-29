#![allow(dead_code)]
use std::collections::VecDeque;

/**
 * Priority Queue aka MinHeap
 * Binary Tree-like data structure
 * Balanced: at most the height difference of nodes will be 1
 * Has heap condition:
 * in MinHeap: every Node under current Node is larger
 * in MaxHeap: every Node under current Node is smaller
 *  the root O(log N)
 * Push to the bottom - bubbles up to find correct place, O(log N)
 */
#[derive(Debug)]
struct PriorityQueue<T> {
    queue: VecDeque<T>,
}

impl<T: Default + PartialOrd> PriorityQueue<T> {
    pub fn new() -> Self {
        let mut queue = VecDeque::with_capacity(1);
        queue.push_back(T::default());
        PriorityQueue {
            queue,
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len() - 1
    }

    pub fn peak(&self) -> Option<&T> {
        if self.len() > 0 {
            Some(&self.queue[1])
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else if self.len() == 1 {
            self.queue.remove(1)
        } else {
            self.queue.swap(1, self.len());
            let v = self.queue.pop_back();
            self.heapify_down(1);
            v
        }
    }

    pub fn push(&mut self, value: T) {
        let i = self.queue.len();
        self.queue.push_back(value);
        self.heapify_up(i);
    }

    fn left(&self, i: usize) -> (usize, Option<&T>) {
        let l_i = i * 2;
        (l_i, self.queue.get(l_i))
    }

    fn right(&self, i: usize) -> (usize, Option<&T>) {
        let r_i = (i * 2) + 1;
        (r_i, self.queue.get(r_i))
    }

    fn min_child (&self, i: usize) -> (usize, Option<&T>) {
        let (l_i, l) = self.left(i);
        let (r_i, r) = self.right(i);

        match (l, r) {
            (Some(l), Some(r)) if l > r => (r_i, Some(r)),
            (Some(l), _) => (l_i, Some(l)),
            _ => (0, None),
        }
    }

    /// Panics if i is out of bounds
    fn heapify_down(&mut self, i: usize) {
        let v = &self.queue[i];
        // if min child is < v, swap with min child
        if let (c_i, Some(child)) = self.min_child(i) {
            if child < v {
                self.queue.swap(c_i, i);
                self.heapify_down(c_i);
            }
        }
    }

    fn heapify_up(&mut self, i: usize) {
        let parent_i = i/2;
        if parent_i != 0 {
            // compare to parent recursively and (maybe) swap
            if self.queue[i] < self.queue[parent_i] {
                self.queue.swap(i, parent_i);
                self.heapify_up(parent_i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pq_works() {
        let mut pq = PriorityQueue::new();
        assert_eq!(None, pq.peak());
        assert_eq!(None, pq.pop());
        pq.push(50);
        assert_eq!(Some(&50), pq.peak());
        pq.push(75);
        pq.push(100);
        dbg!(&pq);
        assert_eq!(Some(&50), pq.peak());
        pq.push(30);
        dbg!(&pq);
        assert_eq!(Some(&30), pq.peak());
        assert_eq!(Some(30), pq.pop());
        dbg!(&pq);
        assert_eq!(Some(50), pq.pop());
        assert_eq!(Some(75), pq.pop());
        assert_eq!(Some(100), pq.pop());
        assert_eq!(None, pq.pop());
    }

}
