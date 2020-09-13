use crate::RingBox::{Root, Box};

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
        match self.buffer.len() {
            0 => unreachable!(),
            1 => self.buffer = vec![Root{prev:1, next:1}, Box{prev:0, next:0, item}],
            len => {
                if let Root{next, ..} = self.buffer[0] {
                    self.buffer.push(Box{prev:0, next, item});
                    if let Some(Box{prev, ..}) = self.buffer.get_mut(next) {
                        *prev = len;
                    }
                    if let Some(Root{next, ..}) = self.buffer.get_mut(0) {
                        *next = len;
                    }
                }
            }
        }
    }

    fn poll(&mut self) -> Option<T> {
        match self.buffer.len() {
            0 => unreachable!(),
            1 => (),
            2 => {
                if let Box{item, ..} = self.buffer.remove(1) {
                    self.buffer = vec![Root{prev:0, next:0}];
                    return Some(item)
                }
            }
            _ => {
                if let Root { prev, .. } = self.buffer[0] {
                    if let Box { item, prev: bprev, next: bnext } = self.buffer.swap_remove(prev) {
                        match self.buffer.get_mut(bnext) {
                            Some(Root { prev: p, .. }) => {
                                *p = bprev;
                            }
                            Some(Box { prev: p, .. }) => {
                                *p = bprev;
                            }
                            _ => unreachable!()
                        }
                        match self.buffer.get_mut(bprev) {
                            Some(Root { next: n, .. }) => {
                                *n = bnext;
                            }
                            Some(Box { next: n, .. }) => {
                                *n = bnext;
                            }
                            _ => unreachable!()
                        }
                        if let Some(&Box { prev: lprev, next: lnext, .. }) = self.buffer.get(prev) {
                            match self.buffer.get_mut(lprev) {
                                Some(Root { next: n, .. }) => {
                                    *n = prev;
                                }
                                Some(Box { next: n, .. }) => {
                                    *n = prev;
                                }
                                _ => unreachable!()
                            }
                            match self.buffer.get_mut(lnext) {
                                Some(Root { prev: p, .. }) => {
                                    *p = prev;
                                }
                                Some(Box { prev: p, .. }) => {
                                    *p = prev;
                                }
                                _ => unreachable!()
                            }
                        }
                        return Some(item)
                    }
                }
            }
        }
        None
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

    #[test]
    fn poll() {
        let mut ring = Ring::new();
        assert_eq!(ring.poll(), None);
        ring.push(1);
        ring.push(2);
        ring.push(3);
        ring.push(4);
        ring.push(5);
        assert_eq!(ring.poll(), Some(1));
        assert_eq!(ring.poll(), Some(2));
        assert_eq!(ring.poll(), Some(3));
        assert_eq!(ring.poll(), Some(4));
        assert_eq!(ring.poll(), Some(5));
        assert_eq!(ring.poll(), None);
    }

    #[test]
    fn push_poll() {
        let mut ring = Ring::new();
        assert_eq!(ring.poll(), None);
        ring.push(1);
        ring.push(2);
        ring.push(3);
        assert_eq!(ring.poll(), Some(1));
        assert_eq!(ring.poll(), Some(2));
        ring.push(4);
        ring.push(5);
        assert_eq!(ring.poll(), Some(3));
        assert_eq!(ring.poll(), Some(4));
        assert_eq!(ring.poll(), Some(5));
        assert_eq!(ring.poll(), None);
    }
}
