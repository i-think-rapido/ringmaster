use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use crate::{Buffer, clone_slices, Snapshot};
use crate::BufferType;
use crate::BufferMode;
use crate::Peek;
use crate::utils::clone;
use super::RingTrait;

#[derive(Default)]
pub(crate) struct RingStorageNaive<T> {
    buffer: RefCell<VecDeque<T>>,
    mode: Cell<BufferType>,
}
impl<T> RingTrait<T, BufferType> for RingStorageNaive<T> {}
impl<T> From<Vec<T>> for RingStorageNaive<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            buffer: RefCell::new(VecDeque::from(vec)),
            mode: Cell::default(),
        }
    }
}
impl<T> Buffer for RingStorageNaive<T> {
    type Item = T;
    fn push(&self, item: T) {
        self.buffer.borrow_mut().push_back(item);
    }
    fn pop(&self) -> Option<T> {
        match self.mode.get() {
            BufferType::FIFO => self.buffer.borrow_mut().pop_front(),
            BufferType::LIFO => self.buffer.borrow_mut().pop_back(),
        }
    }
    fn len(&self) -> usize {
        self.buffer.borrow().len()
    }
    fn is_empty(&self) -> bool {
        self.buffer.borrow().is_empty()
    }
}
impl<T> BufferMode for RingStorageNaive<T> {
    type Mode = BufferType;
    fn mode(&self) -> BufferType {
        self.mode.get()
    }
    fn set_mode(&self, mode: BufferType) {
        self.mode.set(mode);
    }
}
impl<T> Peek for RingStorageNaive<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        let clone = |r| unsafe { clone(r) };
        match self.mode.borrow().get() {
            BufferType::FIFO => self.buffer.borrow().front().map(clone),
            BufferType::LIFO => self.buffer.borrow().back().map(clone),
        }
    }
}

impl<T> Snapshot for RingStorageNaive<T> {
    type Item = T;
    fn snapshot(&self) -> Vec<Self::Item> {
        let buf = self.buffer.borrow();
        let (head, tail) = buf.as_slices();
        let out: Vec<Self::Item> = clone_slices!(head, tail);
        out
    }
}
