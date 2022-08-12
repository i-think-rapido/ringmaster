
use ringmaster::*;

#[test]
fn is_empty() {
    let ring = Ring::<u8>::default();
    assert!(ring.is_empty());
}

#[test]
fn length() {
    let ring = Ring::from(vec![1,2,3,4,5]);
    assert_eq!(ring.len(), 5);
}

#[test]
fn peek() {
    let ring = Ring::default();
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
    let ring = Ring::default();
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
    let ring = Ring::default();
    ring.mode(Mode::LIFO);
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
