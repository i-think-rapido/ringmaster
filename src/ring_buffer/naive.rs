
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use crate::Buffer;
use crate::BufferType;
use crate::BufferMode;
use crate::Peek;
use super::RingTrait;

#[derive(Default)]
pub(crate) struct RingStorageNaive<T: Clone> {
    buffer: RefCell<VecDeque<T>>,
    mode: Cell<BufferType>,
}
impl<T: Clone + 'static> RingTrait<T, BufferType> for RingStorageNaive<T> {}
impl<T: Clone + 'static> From<Vec<T>> for RingStorageNaive<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            buffer: RefCell::new(VecDeque::from(vec)),
            mode: Cell::default(),
        }
    }
}
impl<T: Clone + 'static> Buffer for RingStorageNaive<T> {
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
impl<T: Clone + 'static> BufferMode for RingStorageNaive<T> {
    type Mode = BufferType;
    fn mode(&self) -> BufferType {
        self.mode.get()
    }
    fn set_mode(&self, mode: BufferType) {
        self.mode.set(mode);
    }
}
impl<T: Clone + 'static> Peek for RingStorageNaive<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        match self.mode.borrow().get() {
            BufferType::FIFO => self.buffer.borrow().front().cloned(),
            BufferType::LIFO => self.buffer.borrow().back().cloned(),
        }
    }
}

