use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::sync::RwLock;
use std::convert::From;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BufferType {
    FIFO,
    LIFO,
}
impl Default for BufferType {
    fn default() -> Self {
        BufferType::FIFO
    }
}

pub trait Buffer {
    type Item;
    fn push(&self, item: Self::Item);
    fn pop(&self) -> Option<Self::Item>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
pub trait BufferMode {
    type Mode;
    fn mode(&self) -> Self::Mode;
    fn set_mode(&self, mode: Self::Mode);
}
pub trait Peek {
    type Item;
    fn peek(&self) -> Option<Self::Item>;
}
#[derive(Default)]
pub struct Ring<T: Clone> {
    storage: RwLock<RingStorage<T>>,
}
impl<T: Clone> From<Vec<T>> for Ring<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            storage: RwLock::new(RingStorage {
                buffer: RefCell::new(VecDeque::from(vec)),
                mode: Cell::default(),
            })
        }
    }
}
impl<T: Clone> Buffer for Ring<T> {
    type Item = T;
    fn push(&self, item: T) {
        self.storage.write().unwrap().push(item);
    }
    fn pop(&self) -> Option<T> {
        self.storage.write().unwrap().pop()
    }
    fn len(&self) -> usize {
        self.storage.read().unwrap().len()
    }
    fn is_empty(&self) -> bool {
        self.storage.read().unwrap().is_empty()
    }
}
impl<T: Clone> BufferMode for Ring<T> {
    type Mode = BufferType;
    fn mode(&self) -> BufferType {
        self.storage.read().unwrap().mode()
    }
    fn set_mode(&self, mode: BufferType) {
        self.storage.write().unwrap().set_mode(mode);
    }
}
impl<T: Clone> Peek for Ring<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        self.storage.read().unwrap().peek()
    }
}

#[derive(Default)]
struct RingStorage<T: Clone> {
    buffer: RefCell<VecDeque<T>>,
    mode: Cell<BufferType>,
}
impl<T: Clone> Buffer for RingStorage<T> {
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
impl<T: Clone> BufferMode for RingStorage<T> {
    type Mode = BufferType;
    fn mode(&self) -> BufferType {
        self.mode.get()
    }
    fn set_mode(&self, mode: BufferType) {
        self.mode.set(mode);
    }
}
impl<T: Clone> Peek for RingStorage<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        match self.mode.borrow().get() {
            BufferType::FIFO => self.buffer.borrow().front().cloned(),
            BufferType::LIFO => self.buffer.borrow().back().cloned(),
        }
    }
}

