
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
