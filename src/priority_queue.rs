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
            let end = self.len() - 1;
            self.queue.swap(1, end);
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

    // child
    // l (i * 2)
    // r (i * 2) + 1
    // parent
    // (i/2)
    fn heapify_down(&mut self, i: usize) {
        let v = self.queue.get(i);
        let l = self.queue.get(i * 2);
        let r = self.queue.get((i * 2) + 1);
        // find min child
        // if min child is < v, swap with min child
        match (v, l, r) {
            (Some(v), Some(l), _) if l < v => {
            },
            (Some(v), _, Some(r)) if r < v => {

            },
            _ => (),
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
    fn push_works() {
        let mut pq = PriorityQueue::new();
        assert_eq!(None, pq.peak());
        pq.push(50);
        assert_eq!(Some(&50), pq.peak());
        pq.push(75);
        pq.push(100);
        assert_eq!(Some(&50), pq.peak());
        dbg!(&pq);
        pq.push(30);
        dbg!(&pq);
    }

}
