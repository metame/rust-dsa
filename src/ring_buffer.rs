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
use std::ops::{Index, IndexMut};
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

        let raw_ptr = unsafe { std::alloc::alloc(layout) };

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

    /// panics if index is out of bounds
    fn offset(&self, index: usize) -> usize {
        (self.head.expect("index out of bounds") + index) % self.cap
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
        } else if is_overwrite {
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
        unsafe { Some(ptr::read(self.ptr.as_ptr().add(old_tail))) }
    }

    pub fn iter(&self) -> RingBufferIterator<T> {
        RingBufferIterator {
            current: 0,
            buf: self,
        }
    }
}

impl<T> Index<usize> for RingBuffer<T> {
    type Output = T;
    /// Panics if index is out of bounds
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.length {
            unsafe { self.ptr.as_ptr().add(self.offset(index)).as_ref().expect("NPE how?") }
        } else {
            panic!("index out of bounds")
        }
    }
}

impl<T> IndexMut<usize> for RingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.length {
            unsafe { self.ptr.as_ptr().add(self.offset(index)).as_mut().expect("NPE how?") }
        } else {
            panic!("index out of bounds")
        }
    }
}

pub struct RingBufferIterator<'a, T> {
    current: usize,
    buf: &'a RingBuffer<T>,
}

impl<'a, T: std::fmt::Debug> Iterator for RingBufferIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.buf.length {
            let current = self.current;
            self.current += 1;
            Some(&self.buf[current])
        } else {
            None
        }
    }
}

impl<T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        while let Some(i) = self.pop_back() {
            drop(i);
        }
        unsafe {
            std::alloc::dealloc(
                self.ptr.as_ptr() as *mut u8,
                Layout::array::<T>(self.cap).unwrap(),
            );
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
        assert_eq!(15, r[0]);
        r.push_front(10);
        r.push_front(7);
        r.push_front(-5);
        r.push_front(75);
        r.push_front(-2340);
        assert_eq!(-2340, r[0]);
        assert_eq!(10, r[4]);
        assert_eq!(7, r[3]);
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
    fn index_mut_works() {
        let mut r = RingBuffer::<isize>::new(5);
        r.push_front(24);
        r.push_front(-20);
        r.push_front(-245);
        assert_eq!(24, r[2]);
        r[2] = -45;
        assert_eq!(-45, r[2]);
        assert_eq!(Some(-45), r.pop_back());
        for i in 0..r.length {
            dbg!(r[i]);
        }
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
        assert_eq!(Box::new("gutentag".to_string()), r[2]);
    }

    #[test]
    fn iter_tests() {
        let mut r = RingBuffer::<f64>::new(5);
        r.push_front(5.5);
        r.push_front(8.2);
        r.push_front(9.1);
        r.push_front(6.3);
        r.push_front(1.5436);

        let mut i = 0;
        for e in r.iter() {
            dbg!(e);
            i += 1;
        }

        assert_eq!(i, r.length);

        let mut iter = r.iter();
        assert_eq!(Some(&5.5), iter.nth(4));
        assert!(r.iter().fold(0f64, |x, y| x + y) > 0f64);
    }
}
