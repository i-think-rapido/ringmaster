use crate::RingBox::Root;

enum RingBox<T> {
    Root{prev: usize, next: usize},
    Item{prev: usize, next: usize, payload: T},
}

struct Ring<T> {
    buffer: Vec<RingBox<T>>,
}

impl<T> Ring<T> {

    fn new() -> Ring<T> {
        Ring {
            buffer: vec![Root {prev: 0, next: 0}],
        }
    }

    fn is_empty(&self) -> bool {
        if let Root{prev, next} = self.buffer[0] {
            prev == next
        }
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_ring() {
        let ring = Ring::<u8>::new();
        assert!(ring.is_empty());
    }
}
