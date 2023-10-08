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

        unsafe {
            // TODO: this doesn't handle Ts in tail that should be Dropped if overwriting tail
            ptr::write(self.ptr.as_ptr().add(new_head), item);
        }

        // update head
        self.head = Some(new_head);
        // update tail if length == cap
        if self.length == 0 {
            self.tail = Some(new_head);
        } else if self.length == self.cap {
            self.tail.map(|i| self.before(i));
        }
        // update length if length < cap
        if self.length < self.cap {
            self.length += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // get el at tail
        let old_tail = if self.tail.take().is_some() {
            self.tail.unwrap()
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
        // TODO: call drop on all elements
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

        dbg!(&r);
        r.push_front(15);
        r.push_front(10);
        r.push_front(7);
        r.push_front(-5);
        assert_eq!(Some(15), r.pop_back());
    }
}
