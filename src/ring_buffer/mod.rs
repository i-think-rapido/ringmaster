
mod naive;

use crate::enums::BufferType;
use crate::traits::*;
use std::sync::{Arc, RwLock};
use std::convert::From;

use self::naive::RingStorageNaive;

pub(crate) trait RingTrait<T, M> : Buffer<Item=T> + BufferMode<Mode=M> + Peek<Item=T> + Snapshot<Item=T>{}
#[derive(Clone)]
pub struct RingBuffer<T> {
    storage: Arc<RwLock<dyn RingTrait<T, BufferType>>>,
}
impl<T: 'static> RingBuffer<T> {
    pub fn new() -> Self {
        Self::from(vec![])
    }
}
impl<T: 'static> From<Vec<T>> for RingBuffer<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            storage: Arc::new(RwLock::new(RingStorageNaive::from(vec))),
        }
    }
}
impl<T> Buffer for RingBuffer<T> {
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
impl<T> BufferMode for RingBuffer<T> {
    type Mode = BufferType;
    fn mode(&self) -> BufferType {
        self.storage.read().unwrap().mode()
    }
    fn set_mode(&self, mode: BufferType) {
        self.storage.write().unwrap().set_mode(mode);
    }
}
impl<T> Peek for RingBuffer<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        self.storage.read().unwrap().peek()
    }
}

impl<T> Snapshot for RingBuffer<T> {
    type Item = T;
    fn snapshot(&self) -> Vec<Self::Item> {
        self.storage.read().unwrap().snapshot()
    }
}
