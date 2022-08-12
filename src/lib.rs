mod ring_storage;

use std::sync::{Arc, RwLock};
use std::convert::From;
use crate::ring_storage::*;

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

pub(crate) trait RingTrait<T, M> : Buffer<Item=T> + BufferMode<Mode=M> + Peek<Item=T> + 'static {}
#[derive(Clone)]
pub struct Ring<T: Clone> {
    storage: Arc<RwLock<dyn RingTrait<T, BufferType>>>,
}
impl<T: Clone + 'static> Ring<T> {
    pub fn new() -> Self {
        Self::from(vec![])
    }
}
impl<T: Clone + 'static> From<Vec<T>> for Ring<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            storage: Arc::new(RwLock::new(RingStorageNaive::from(vec))),
        }
    }
}
impl<T: Clone + 'static> Buffer for Ring<T> {
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
impl<T: Clone + 'static> BufferMode for Ring<T> {
    type Mode = BufferType;
    fn mode(&self) -> BufferType {
        self.storage.read().unwrap().mode()
    }
    fn set_mode(&self, mode: BufferType) {
        self.storage.write().unwrap().set_mode(mode);
    }
}
impl<T: Clone + 'static> Peek for Ring<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        self.storage.read().unwrap().peek()
    }
}

