
mod storage;

use std::sync::{Arc, RwLock};
use crate::traits::*;
use self::storage::TimeseriesStorage;

#[derive(Default, Clone)]
pub struct Timeseries<T> {
    storage: Arc<RwLock<TimeseriesStorage<T>>>
}

impl<T: Clone + 'static> Timeseries<T> {
    pub fn new() -> Self {
        Self::from(vec![])
    }
}
impl<T: Clone + 'static> From<Vec<T>> for Timeseries<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            storage: Arc::new(RwLock::new(TimeseriesStorage::from(vec))),
        }
    }
}
impl<T: Clone + 'static> Buffer for Timeseries<T> {
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
impl<T: Clone + 'static> Peek for Timeseries<T> {
    type Item = T;
    fn peek(&self) -> Option<Self::Item> {
        self.storage.read().unwrap().peek()
    }
}

impl<T> Capacity for Timeseries<T> {
    fn with_capacity(cap: usize) -> Self {
        Self {
            storage: Arc::new(RwLock::new(TimeseriesStorage::with_capacity(cap))),
        }
    }
    fn capacity(&self) -> usize {
        self.storage.read().unwrap().capacity()
    }
}

impl<T: Copy> Snapshot for Timeseries<T> {
    type Item = T;
    fn snapshot(&self) -> Vec<Self::Item> {
        self.storage.read().unwrap().snapshot()
    }
}
impl<T: Copy> SnapshotRaw for Timeseries<T> {
    type Item = T;
    unsafe fn snapshot_raw(&self) -> Vec<Self::Item> {
        self.storage.read().unwrap().snapshot_raw()
    }
}
