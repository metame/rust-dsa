#![allow(dead_code)]
/*
Stack:
 * LiFo queue
 * pop/push of O(1)
 */
use std::alloc::{self, Layout};
use std::mem;
use std::ptr::{self, NonNull};

pub struct Stack<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Stack<T> {}
unsafe impl<T: Sync> Sync for Stack<T> {}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    fn grow(&mut self) {
        let (new_cap, layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;

            assert!(new_cap <= isize::MAX as usize, "Allocation too large!!!!!");

            (new_cap, Layout::array::<T>(self.cap).unwrap())
        };

        let ptr = if self.cap == 0 {
            unsafe {
                alloc::alloc(layout)
            }
        } else {
            let new_size = new_cap * mem::size_of::<T>();
            unsafe {
                alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size)
            }
        };

        self.cap = new_cap;
        match NonNull::new(ptr as *mut T) {
            Some(ptr) => { self.ptr = ptr; },
            None => alloc::handle_alloc_error(layout),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    pub fn push(&mut self, item: T) { // item = 1, len = 5, cap = 8
        if self.cap == self.len {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), item);
        }
        self.len += 1;
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while let Some(item) = self.pop() {
            drop(item);
        }
        unsafe {
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, Layout::array::<T>(self.cap).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_works() {
        let mut s = Stack::<usize>::new();
        assert_eq!(None, s.pop());
        assert_eq!(0, s.len);
        s.push(5);
        assert_eq!(1, s.len);
        assert_eq!(Some(5), s.pop());
        s.push(4);
        assert_eq!(1, s.cap);
        s.push(10);
        assert_eq!(2, s.len);
        for n in 1..=1000 {
            s.push(n);
        }
        for _ in 1..=10 {
            s.pop();
        }
    }
}
