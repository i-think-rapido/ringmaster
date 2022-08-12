use std::cell::RefCell;
use std::collections::VecDeque;
use crate::clone_slices;
use crate::traits::*;

#[derive(Default)]
pub struct TimeseriesStorage<T> {
    buf: RefCell<VecDeque<T>>,
}
impl<T: Clone + 'static> From<Vec<T>> for TimeseriesStorage<T> {
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

impl<T> Snapshot for TimeseriesStorage<T> {
    type Item = T;
    fn snapshot(&self) -> Vec<Self::Item> {
        let buf = self.buf.borrow();
        let (head, tail) = buf.as_slices();
        let out: Vec<Self::Item> = clone_slices!(head, tail);
        out
    }
}
