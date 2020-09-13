use crate::RingBox::{Root, Box};

#[derive(Debug)]
enum RingBox<T> {
    Root{ prev: usize, next: usize },
    Box{ prev: usize, next: usize, item: T},
}

#[derive(Default)]
struct Ring<T> {
    buffer: Vec<RingBox<T>>,
}

impl<T> Ring<T> {

    fn new() -> Ring<T> {
        Ring {
            buffer: vec![Root{prev: 0, next: 0}],
        }
    }

    fn is_empty(&self) -> bool {
        self.buffer.len() == 1
    }

    fn push(&mut self, item: T) {
        if let Root{prev, next} = self.buffer[0] {
            match (prev, next) {
                (0, 0) => {
                    if let Some(Root{prev, next}) = self.buffer.get_mut(0) {
                        *prev = 1;
                        *next = 1;
                    }
                    self.buffer.push(Box{prev: 0, next: 0, item});
                }
                (_, last) => {
                    let new = self.buffer.len();
                    if let Some(Box{prev, ..}) = self.buffer.get_mut(last) {
                        *prev = new;
                    }
                    if let Some(Root{next, ..}) = self.buffer.get_mut(0) {
                        *next = new;
                    }
                    self.buffer.push(Box{prev: 0, next: last, item});
                }
            }
        }
    }

}

impl<T> From<Vec<T>> for Ring<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut ring = Ring::new();
        for item in vec.into_iter() {
            ring.push(item);
        }
        ring
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_empty() {
        let ring = Ring::<u8>::new();
        assert!(ring.is_empty());
    }

    #[test]
    fn push() {
        let mut ring = Ring::new();
        ring.push(1);
        if let Root{prev, next} = ring.buffer[0] {
            assert_eq!(prev, 1);
            assert_eq!(next, 1);
        }
        if let Some(&Box{prev, next, item}) = ring.buffer.last() {
            assert_eq!(prev, 0);
            assert_eq!(next, 0);
            assert_eq!(item, 1);
        }
    }

    #[test]
    fn from() {
        let ring = Ring::from(vec![1,3,4,6]);
        if let Some(&Box{prev, next, item}) = ring.buffer.last() {
            assert_eq!(prev, 0);
            assert_eq!(next, 3);
            assert_eq!(item, 6);
        }
        if let Box{prev, next, item} = ring.buffer[1] {
            assert_eq!(prev, 2);
            assert_eq!(next, 0);
            assert_eq!(item, 1);
        }
    }
}
