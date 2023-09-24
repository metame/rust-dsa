#![allow(dead_code)]
/*

 * Wrapper that uses arrays under the hood
 * push/pop/access has O(1)
 * enqueue/deque has O(N)
 * constructor specifies initial size


TypeScript example:
export default class ArrayList<T> {
    public length: number;
    constructor() {}
    prepend(item: T): void {}
    insertAt(item: T, idx: number): void {}
    append(item: T): void {}
    remove(item: T): T | undefined {}
    get(idx: number): T | undefined {}
    removeAt(idx: number): T | undefined {}
}
 */
use std::fmt::Debug;
// My Fake Array, we're pretending all we can do is get length and not grow this
type Array<T> = Vec<T>;

#[derive(Debug)]
struct ArrayList<T> {
    pub length: usize,
    inner: Array<T>,
}

impl<T: Debug + Default + Clone + PartialEq> ArrayList<T> {
    pub fn new() -> ArrayList<T> {
        ArrayList {
            length: 0,
            inner: vec![T::default(); 5],
        }
    }

    fn grow_inner(&mut self) {
        let prev = &self.inner;
        let mut new = vec![T::default(); prev.len() * 2];
        for i in 0..prev.len() {
            new[i] = prev[i].clone();
        }
        self.inner = new;
    }

    pub fn append(&mut self, item: T) {
        println!("appending {item:?} to {self:?}");
        if self.inner.len() == self.length {
            self.grow_inner();
        }
        self.inner[self.length] = item;
        self.length = self.length + 1;
    }

    // TODO: handle 0 elements case
    pub fn pop(&mut self) -> T {
        let tail = self.length - 1;
        let item = self.inner[tail].clone();
        println!("removing {item:?} from {self:?}");
        self.inner[tail] = T::default();
        self.length = tail;
        item
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            let item = self.inner[index].clone();
            for i in index..self.length {
                self.inner[i] = self.inner[i + 1].clone();
            }
            self.length = self.length - 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn remove(&mut self, item: &T) -> Option<T> {
        for i in 0..self.length {
            if &self.inner[i] == item {
                return self.remove_at(i);
            }
        }
        None
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.length {
            Some(&self.inner[i])
        } else {
            None
        }
    }

    pub fn insert_at(&mut self, index: usize, item: T) {
        println!("inserting {item:?} at {index:?} into {self:?}");
        if self.inner.len() == self.length {
            self.grow_inner();
        }
        for i in ((index + 1)..=self.length).rev() {
            self.inner[i] = self.inner[i - 1].clone();
        }
        self.inner[index] = item;
        self.length = self.length + 1;
    }

    pub fn prepend(&mut self, item: T) {
        println!("prepending {item:?} to {self:?}");
        self.insert_at(0, item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_list_works() {
        let mut l = ArrayList::<i32>::new();
        l.append(1);
        l.append(2);
        l.append(3);
        l.append(4);
        l.append(5);
        l.append(-1);
        assert_eq!(Some(&5), l.get(4));
        let item = l.pop();
        assert_eq!(-1, item);
        assert_eq!(5, l.length);
        assert_eq!(Some(&3), l.get(2));
        assert_eq!(None, l.get(l.length));
        l.prepend(1000);
        assert_eq!(Some(&1000), l.get(0));
        assert_eq!(Some(&1), l.get(1));
        assert_eq!(Some(&2), l.get(2));
        assert_eq!(Some(&5), l.get(l.length - 1));
        assert_eq!(6, l.length);
        l.insert_at(1, 2000);
        assert_eq!(Some(&2000), l.get(1));
        assert_eq!(Some(2000), l.remove_at(1));
        assert_eq!(6, l.length);
        assert_eq!(Some(&5), l.get(l.length - 1));
        assert_eq!(Some(3), l.remove(&3));
        assert_eq!(None, l.remove(&500));
        assert_eq!(5, l.length);
    }
}
