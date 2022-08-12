use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::sync::RwLock;
use std::convert::From;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    FIFO,
    LIFO,
}
impl Default for Mode {
    fn default() -> Self {
        Mode::FIFO
    }
}

pub trait Buffer {
    type Item;
    fn push(&self, item: Self::Item);
    fn pop(&self) -> Option<Self::Item>;
    fn mode(&self) -> Mode;
    fn set_mode(&self, mode: Mode);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
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
    fn mode(&self) -> Mode {
        self.storage.read().unwrap().mode()
    }
    fn set_mode(&self, mode: Mode) {
        self.storage.write().unwrap().set_mode(mode);
    }
    fn len(&self) -> usize {
        self.storage.read().unwrap().len()
    }
    fn is_empty(&self) -> bool {
        self.storage.read().unwrap().is_empty()
    }

    fn peek(&self) -> Option<Self::Item> {
        self.storage.read().unwrap().peek()
    }
}

#[derive(Default)]
struct RingStorage<T: Clone> {
    buffer: RefCell<VecDeque<T>>,
    mode: Cell<Mode>,
}
impl<T: Clone> Buffer for RingStorage<T> {
    type Item = T;
    fn push(&self, item: T) {
        self.buffer.borrow_mut().push_back(item);
    }
    fn pop(&self) -> Option<T> {
        match self.mode.get() {
            Mode::FIFO => self.buffer.borrow_mut().pop_front(),
            Mode::LIFO => self.buffer.borrow_mut().pop_back(),
        }
    }
    fn mode(&self) -> Mode {
        self.mode.get()
    }
    fn set_mode(&self, mode: Mode) {
        self.mode.set(mode);
    }
    fn len(&self) -> usize {
        self.buffer.borrow().len()
    }
    fn is_empty(&self) -> bool {
        self.buffer.borrow().is_empty()
    }

    fn peek(&self) -> Option<Self::Item> {
        match self.mode.borrow().get() {
            Mode::FIFO => self.buffer.borrow().front().cloned(),
            Mode::LIFO => self.buffer.borrow().back().cloned(),
        }
    }
}

