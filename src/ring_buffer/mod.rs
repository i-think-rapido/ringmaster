
mod naive;

use crate::enums::BufferType;
use crate::traits::*;
use std::sync::{Arc, RwLock};
use std::convert::From;

pub use self::naive::RingStorageNaive;

#[derive(Default, Clone)]
pub struct RingBuffer<Impl>
{
    storage: Arc<RwLock<Impl>>,
}
impl<T, Impl: From<Vec<T>>> From<Vec<T>> for RingBuffer<Impl> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            storage: Arc::new(RwLock::new(Impl::from(vec))),
        }
    }
}
impl<T, Impl: Buffer<Item=T>> Buffer for RingBuffer<Impl> {
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
impl<Impl: BufferMode<Mode=BufferType>> BufferMode for RingBuffer<Impl> {
    type Mode = BufferType;
    fn mode(&self) -> BufferType {
        self.storage.read().unwrap().mode()
    }
    fn set_mode(&self, mode: BufferType) {
        self.storage.write().unwrap().set_mode(mode);
    }
}
impl<T, Impl: Peek<Item=T>> Peek for RingBuffer<Impl> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        self.storage.read().unwrap().peek()
    }
}

impl<T, Impl: Snapshot<Item=T>> Snapshot for RingBuffer<Impl> {
    type Item = T;
    fn snapshot(&self) -> Vec<Self::Item> {
        self.storage.read().unwrap().snapshot()
    }
}
impl<T: Copy, Impl: SnapshotRaw<Item=T>> SnapshotRaw for RingBuffer<Impl> {
    type Item = T;
    unsafe fn snapshot_raw(&self) -> Vec<std::mem::MaybeUninit<Self::Item>> {
        self.storage.read().unwrap().snapshot_raw()
    }
}
