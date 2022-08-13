use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use crate::{Buffer, unsafe_clone_slices, SnapshotRaw, Snapshot};
use crate::BufferType;
use crate::BufferMode;
use crate::Peek;

#[derive(Default)]
pub struct RingStorageNaive<T> {
    buffer: RefCell<VecDeque<T>>,
    mode: Cell<BufferType>,
}
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
impl<T: Clone> Peek for RingStorageNaive<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        match self.mode.borrow().get() {
            BufferType::FIFO => self.buffer.borrow().front().cloned(),
            BufferType::LIFO => self.buffer.borrow().back().cloned(),
        }
    }
}

impl<T: Clone> Snapshot for RingStorageNaive<T> {
    type Item = T;
    fn snapshot(&self) -> Vec<Self::Item> {
        let mut out = vec![];
        let buf = self.buffer.borrow();
        let (head, tail) = buf.as_slices();
        out.append(&mut head.to_vec());
        out.append(&mut tail.to_vec());
        out
    }
}
impl<T: Copy> SnapshotRaw for RingStorageNaive<T> {
    type Item = T;
    unsafe fn snapshot_raw(&self) -> Vec<Self::Item> {
        let buf = self.buffer.borrow();
        let (head, tail) = buf.as_slices();
        let out: Vec<Self::Item> = unsafe_clone_slices!(head, tail);
        out
    }
}
