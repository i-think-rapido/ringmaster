
use ringmaster::*;
use ringmaster::ring_buffer::RingStorageNaive;

#[test]
fn is_empty() {
    let ring = RingBuffer::<RingStorageNaive<i32>>::default();
    assert!(ring.is_empty());
}

#[test]
fn len() {
    let ring = RingBuffer::<RingStorageNaive<i32>>::from(vec![1, 2, 3, 4, 5]);
    assert_eq!(ring.len(), 5);
}

#[test]
fn mode() {
    let ring = RingBuffer::<RingStorageNaive<i32>>::default();
    assert!(ring.mode() == BufferType::FIFO);
    ring.set_mode(BufferType::LIFO);
    assert!(ring.mode() == BufferType::LIFO);
    ring.set_mode(BufferType::FIFO);
    assert!(ring.mode() == BufferType::FIFO);
}

#[test]
fn peek() {
    let ring = RingBuffer::<RingStorageNaive<i32>>::default();
    assert_eq!(ring.peek(), None);
    ring.push(1);
    ring.push(2);
    assert_eq!(ring.peek(), Some(1));
    ring.pop();
    assert_eq!(ring.peek(), Some(2));
    ring.pop();
    assert_eq!(ring.pop(), None);
}

#[test]
fn push_pop() {
    let ring = RingBuffer::<RingStorageNaive<i32>>::default();
    assert_eq!(ring.peek(), None);
    ring.push(1);
    ring.push(2);
    ring.push(3);
    assert_eq!(ring.pop(), Some(1));
    assert_eq!(ring.pop(), Some(2));
    ring.push(4);
    ring.push(5);
    assert_eq!(ring.pop(), Some(3));
    assert_eq!(ring.pop(), Some(4));
    assert_eq!(ring.pop(), Some(5));
    assert_eq!(ring.pop(), None);
}

#[test]
fn lifo_push_pop() {
    let ring = RingBuffer::<RingStorageNaive<i32>>::default();
    ring.set_mode(BufferType::LIFO);
    assert_eq!(ring.peek(), None);
    ring.push(1);
    assert_eq!(ring.peek(), Some(1));
    ring.push(2);
    assert_eq!(ring.peek(), Some(2));
    ring.push(3);
    assert_eq!(ring.peek(), Some(3));
    assert_eq!(ring.pop(), Some(3));
    assert_eq!(ring.pop(), Some(2));
    ring.push(4);
    assert_eq!(ring.peek(), Some(4));
    ring.push(5);
    assert_eq!(ring.peek(), Some(5));
    assert_eq!(ring.pop(), Some(5));
    assert_eq!(ring.pop(), Some(4));
    assert_eq!(ring.pop(), Some(1));
    assert_eq!(ring.pop(), None);
}

#[test]
fn with_struct() {
    #[derive(Clone, Default)]
    struct A;
    let ring = RingBuffer::<RingStorageNaive<A>>::default();
    ring.push(A);
}

#[test]
fn snapshot_raw() {
    let vec = vec![1, 3, 5];
    let ring = RingBuffer::<RingStorageNaive<i32>>::from(vec);
    ring.push(7);
    ring.push(9);
    unsafe {
        assert_eq!(ring.snapshot_raw(), vec![1, 3, 5, 7, 9]);
    }
}

#[test]
fn snapshot() {
    let vec = vec![1, 3, 5];
    let ring = RingBuffer::<RingStorageNaive<i32>>::from(vec);
    ring.push(7);
    ring.push(9);
    assert_eq!(ring.snapshot(), vec![1, 3, 5, 7, 9]);
}
