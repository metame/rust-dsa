#![allow(dead_code)]
/*
 * RingBuffer
 * Uses an fixed-size array under the hood
 * stores length, index of head, index of tail
 * Array is padded at both head and tail so you can have O(1) benefit on both sides
 * If you exceed the size of array on either side, you can wrap around to the other, that is it's possible for actual index of head > tail
    * e.g. head may be at 4 and tail may be at 1 in a list of 10 elements
    * RingBuffer may describe this as head = 4, tail = 11 and to get actual index of tail you'd do `tail % len` which in the case would be 1
 * push_front
 * pop_back
*/
use std::alloc::Layout;
use std::mem;
use std::ptr::{self, NonNull};

#[derive(std::fmt::Debug)]
pub struct RingBuffer<T> {
    ptr: NonNull<T>,
    head: Option<usize>,
    tail: Option<usize>,
    cap: usize,
    length: usize,
}

impl<T> RingBuffer<T> {
    /// if you allocate more memory than you have, this will panic
    fn allocate(cap: usize) -> NonNull<T> {
        let layout = Layout::array::<T>(cap).unwrap();

        assert!(cap <= isize::MAX as usize, "Allocation too large!!!!!");

        let raw_ptr = unsafe {
            std::alloc::alloc(layout)
        };

        match NonNull::new(raw_ptr as *mut T) {
            Some(ptr) => ptr,
            None => std::alloc::handle_alloc_error(layout),
        }
    }

    pub fn new(cap: usize) -> Self {
        let ptr = Self::allocate(cap);
        RingBuffer {
            ptr,
            head: None,
            tail: None,
            cap,
            length: 0,
        }
    }

    fn before(&self, i: usize) -> usize {
        if i == 0 {
            self.cap - 1
        } else {
            i - 1
        }
    }

    pub fn push_front(&mut self, item: T) {
        // add el to head - 1
        let new_head = if let Some(old_i) = self.head.take() {
            self.before(old_i)
        } else {
            &self.cap / 4
        };

        let is_overwrite = self.length == self.cap;

        unsafe {
            let p = self.ptr.as_ptr().add(new_head);
            if mem::needs_drop::<T>() && is_overwrite {
                let v = ptr::read(p);
                drop(v);
            }
            ptr::write(p, item);
        }

        // update head
        self.head = Some(new_head);
        // update tail if length == cap
        if self.length == 0 {
            self.tail = Some(new_head);
        } else if self.length == self.cap {
            self.tail = self.tail.map(|i| self.before(i));
        }
        // update length if length < cap
        if self.length < self.cap {
            self.length += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // get el at tail
        let old_tail = if let Some(old_i) = self.tail.take() {
            old_i
        } else {
            return None;
        };
        // decrement length
        self.length -= 1;
        // update tail to tail - 1
        if self.length > 0 {
            self.tail = Some(self.before(old_tail));
        }
        // return el
        unsafe {
            Some(ptr::read(self.ptr.as_ptr().add(old_tail)))
        }
    }
}

impl<T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        while let Some(i) = self.pop_back() {
            drop(i);
        }
        unsafe {
            std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, Layout::array::<T>(self.cap).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_buffer_works() {
        let mut r = RingBuffer::<isize>::new(5);

        r.push_front(15);
        r.push_front(10);
        r.push_front(7);
        r.push_front(-5);
        r.push_front(75);
        r.push_front(-2340);
        assert_eq!(Some(10), r.pop_back());
        assert_eq!(Some(7), r.pop_back());
        r.push_front(-3498);
        r.push_front(8383);
        assert_eq!(Some(-5), r.pop_back());
        assert_eq!(Some(75), r.pop_back());
        assert_eq!(Some(-2340), r.pop_back());
        assert_eq!(Some(-3498), r.pop_back());
        assert_eq!(Some(8383), r.pop_back());
        assert_eq!(None, r.pop_back());
    }

    #[test]
    fn box_rb_works() {
        let mut r = RingBuffer::<Box<String>>::new(5);
        r.push_front(Box::new("hello".to_string()));
        r.push_front(Box::new("hey".to_string()));
        r.push_front(Box::new("hola".to_string()));
        r.push_front(Box::new("gutentag".to_string()));
        r.push_front(Box::new("ohaiyo".to_string()));
        r.push_front(Box::new("sup".to_string()));
        assert_eq!(Some("hey".to_string()), r.pop_back().map(|b| *b));
    }
}
