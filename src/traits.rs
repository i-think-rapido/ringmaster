
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

pub trait Capacity {
    fn with_capacity(cap: usize) -> Self;
    fn capacity(&self) -> usize;
}

pub trait Snapshot {
    type Item;
    fn snapshot(&self) -> Vec<Self::Item>;
}
