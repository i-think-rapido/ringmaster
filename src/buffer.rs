
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

