use std::cell::RefCell;
use std::collections::VecDeque;
use crate::unsafe_clone_slices;
use crate::traits::*;

#[derive(Default)]
pub struct TimeseriesStorage<T> {
    buf: RefCell<VecDeque<T>>,
}
impl<T> From<Vec<T>> for TimeseriesStorage<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            buf: RefCell::new(VecDeque::from(vec)),
        }
    }
}

impl<T> Buffer for TimeseriesStorage<T> {
    type Item = T;
    fn push(&self, item: Self::Item) {
        let mut buf = self.buf.borrow_mut();
        if buf.len() == buf.capacity() {
            let _ = buf.pop_back();
            buf.push_front(item)
        } else {
            buf.push_front(item)
        }
    }
    fn pop(&self) -> Option<Self::Item> {
        let mut buf = self.buf.borrow_mut();
        buf.pop_back()
    }
    fn len(&self) -> usize {
        self.buf.borrow().len()
    }
    fn is_empty(&self) -> bool {
        self.buf.borrow().is_empty()
    }
}
impl<T: Clone> Peek for TimeseriesStorage<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        self.buf.borrow().get(0).cloned()
    }
}

impl<T> Capacity for TimeseriesStorage<T> {
    fn with_capacity(cap: usize) -> Self {
        Self {
            buf: RefCell::new(VecDeque::with_capacity(cap)),
        }
    }
    fn capacity(&self) -> usize {
        self.buf.borrow().capacity()
    }
}

impl<T: Copy> Snapshot for TimeseriesStorage<T> {
    type Item = T;
    fn snapshot(&self) -> Vec<Self::Item> {
        let mut out = vec![];
        let buf = self.buf.borrow();
        let (head, tail) = buf.as_slices();
        out.append(&mut head.to_vec());
        out.append(&mut tail.to_vec());
        out
    }
}
impl<T: Copy> SnapshotRaw for TimeseriesStorage<T> {
    type Item = T;
    unsafe fn snapshot_raw(&self) -> Vec<std::mem::MaybeUninit<Self::Item>> {
        let buf = self.buf.borrow();
        let (head, tail) = buf.as_slices();
        let out: Vec<std::mem::MaybeUninit<Self::Item>> = unsafe_clone_slices!(head, tail);
        out
    }
}
