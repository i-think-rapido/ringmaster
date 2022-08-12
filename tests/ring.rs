
use ringmaster::*;

#[test]
fn is_empty() {
    let ring = Ring::<u8>::new();
    assert!(ring.is_empty());
}

#[test]
fn len() {
    let ring = Ring::from(vec![1,2,3,4,5]);
    assert_eq!(ring.len(), 5);
}

#[test]
fn mode() {
    let ring = Ring::<u8>::new();
    assert!(ring.mode() == BufferType::FIFO);
    ring.set_mode(BufferType::LIFO);
    assert!(ring.mode() == BufferType::LIFO);
    ring.set_mode(BufferType::FIFO);
    assert!(ring.mode() == BufferType::FIFO);
}

#[test]
fn peek() {
    let ring = Ring::new();
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
    let ring = Ring::new();
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
    let ring = Ring::new();
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
    let ring = Ring::new();
    #[derive(Clone)]
    struct A;
    ring.push(A);
}