use crate::Slot::{Root, Box};
use crate::Mode::{FIFO, LIFO};
use tokio::sync::Mutex;

enum Slot<T> {
    Root{ prev: usize, next: usize },
    Box{ prev: usize, next: usize, item: T},
}

enum Mode {
    FIFO,
    LIFO,
}
impl Default for Mode {
    fn default() -> Self {
        FIFO
    }
}

#[derive(Default)]
struct Ring<T> {
    buffer: Mutex<Vec<Slot<T>>>,
    mode: Mode,
}

impl<T> Ring<T> {

    fn new() -> Self {
        Self {
            buffer: Mutex::new(vec![Root{prev: 0, next: 0}]),
            mode: FIFO,
        }
    }

    fn as_fifo(self) -> Self {
        Self {
            buffer: self.buffer,
            mode: FIFO,
        }
    }

    fn as_lifo(self) -> Self {
        Self {
            buffer: self.buffer,
            mode: LIFO,
        }
    }

    async fn from(vec: Vec<T>) -> Self {
        let ring = Ring::new();
        for item in vec.into_iter() {
            ring.push(item).await;
        }
        ring
    }

    async fn is_empty(&self) -> bool {
        self.buffer.lock().await.len() == 1
    }

    async fn push(&self, item: T) {
        let mut vec = self.buffer.lock().await;
        match vec.len() {
            0 => unreachable!(),
            1 => {
                vec.clear();
                vec.push(Root{prev:1, next:1});
                vec.push(Box{prev:0, next:0, item});
            },
            len => {
                if let Root{next, ..} = vec[0] {
                    vec.push(Box{prev:0, next, item});
                    if let Some(Box{prev, ..}) = vec.get_mut(next) {
                        *prev = len;
                    }
                    if let Some(Root{next, ..}) = vec.get_mut(0) {
                        *next = len;
                    }
                }
            }
        }
    }

    async fn poll(&self) -> Option<T> {
        let mut vec = self.buffer.lock().await;
        match vec.len() {
            0 => unreachable!(),
            1 => (),
            2 => {
                if let Box{item, ..} = vec.remove(1) {
                    let mut vec = vec;
                    vec.clear();
                    vec.push(Root{prev:0, next:0});
                    return Some(item)
                }
            }
            _ => {
                if let Root { prev, next } = vec[0] {
                    let pos = match self.mode {
                        FIFO => prev,
                        LIFO => next,
                    };
                    if let Box { item, prev: bprev, next: bnext } = vec.swap_remove(pos) {
                        match vec.get_mut(bnext) {
                            Some(Root { prev: p, .. }) => {
                                *p = bprev;
                            }
                            Some(Box { prev: p, .. }) => {
                                *p = bprev;
                            }
                            _ => unreachable!()
                        }
                        match vec.get_mut(bprev) {
                            Some(Root { next: n, .. }) => {
                                *n = bnext;
                            }
                            Some(Box { next: n, .. }) => {
                                *n = bnext;
                            }
                            _ => unreachable!()
                        }
                        if let Some(&Box { prev: lprev, next: lnext, .. }) = vec.get(pos) {
                            match vec.get_mut(lprev) {
                                Some(Root { next: n, .. }) => {
                                    *n = pos;
                                }
                                Some(Box { next: n, .. }) => {
                                    *n = pos;
                                }
                                _ => unreachable!()
                            }
                            match vec.get_mut(lnext) {
                                Some(Root { prev: p, .. }) => {
                                    *p = pos;
                                }
                                Some(Box { prev: p, .. }) => {
                                    *p = pos;
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

    async fn poll_with(&self, f: fn(&T) -> bool) -> Option<T> {
        let vec = self.buffer.lock().await;
        if let Root{prev, next} = vec[0] {

            let pos = match self.mode {
                FIFO => prev,
                LIFO => next,
            };
            if let Some(Box { item, .. }) = vec.get(pos) {
                if f(&item) {
                    drop(vec);
                    return self.poll().await
                }
            }
        }
        None
    }

    async fn peek<R>(&self, f: fn(&T) -> Option<R>) -> Option<R> {
        let vec = self.buffer.lock().await;
        let v = &*vec;
        if let Root{prev, next} = v[0] {

            let pos = match self.mode {
                FIFO => prev,
                LIFO => next,
            };
            if let Some(Box { item, .. }) = v.get(pos) {
                return f(item)
            }
        }
        None
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn is_empty() {
        let ring = Ring::<u8>::new();
        assert!(ring.is_empty().await);
    }

    #[tokio::test]
    async fn push() {
        let ring = Ring::new();
        ring.push(1).await;
        let vec = ring.buffer.lock().await;
        if let Root{prev, next} = vec[0] {
            assert_eq!(prev, 1);
            assert_eq!(next, 1);
        }
        if let Some(&Box{prev, next, item}) = vec.last() {
            assert_eq!(prev, 0);
            assert_eq!(next, 0);
            assert_eq!(item, 1);
        }
    }

    #[tokio::test]
    async fn from() {
        let ring = Ring::from(vec![1,3,4,6]).await;
        let vec = ring.buffer.lock().await;
        if let Some(&Box{prev, next, item}) = vec.last() {
            assert_eq!(prev, 0);
            assert_eq!(next, 3);
            assert_eq!(item, 6);
        }
        if let Box{prev, next, item} = vec[1] {
            assert_eq!(prev, 2);
            assert_eq!(next, 0);
            assert_eq!(item, 1);
        }
    }

    #[tokio::test]
    async fn poll() {
        let ring = Ring::new();
        assert_eq!(ring.poll().await, None);
        ring.push(1).await;
        ring.push(2).await;
        ring.push(3).await;
        ring.push(4).await;
        ring.push(5).await;
        assert_eq!(ring.poll().await, Some(1));
        assert_eq!(ring.poll().await, Some(2));
        assert_eq!(ring.poll().await, Some(3));
        assert_eq!(ring.poll().await, Some(4));
        assert_eq!(ring.poll().await, Some(5));
        assert_eq!(ring.poll().await, None);
    }

    #[tokio::test]
    async fn push_poll() {
        let ring = Ring::new();
        assert_eq!(ring.poll().await, None);
        ring.push(1).await;
        ring.push(2).await;
        ring.push(3).await;
        assert_eq!(ring.poll().await, Some(1));
        assert_eq!(ring.poll().await, Some(2));
        ring.push(4).await;
        ring.push(5).await;
        assert_eq!(ring.poll().await, Some(3));
        assert_eq!(ring.poll().await, Some(4));
        assert_eq!(ring.poll().await, Some(5));
        assert_eq!(ring.poll().await, None);
    }

    #[tokio::test]
    async fn lifo_push_poll() {
        let ring = Ring::new().as_lifo();
        assert_eq!(ring.poll().await, None);
        ring.push(1).await;
        ring.push(2).await;
        ring.push(3).await;
        assert_eq!(ring.poll().await, Some(3));
        assert_eq!(ring.poll().await, Some(2));
        ring.push(4).await;
        ring.push(5).await;
        assert_eq!(ring.poll().await, Some(5));
        assert_eq!(ring.poll().await, Some(4));
        assert_eq!(ring.poll().await, Some(1));
        assert_eq!(ring.poll().await, None);
    }

    #[tokio::test]
    async fn poll_with() {

        let filter = |x: &i32| *x < 3;

        let ring = Ring::new();
        assert_eq!(ring.poll().await, None);
        ring.push(1).await;
        ring.push(2).await;
        ring.push(3).await;
        assert_eq!(ring.poll_with(filter).await, Some(1));
        assert_eq!(ring.poll_with(filter).await, Some(2));
        assert_eq!(ring.poll_with(filter).await, None);
    }

    fn lambda(x: &i32) -> Option<i32> { return Some(*x); }

    #[tokio::test]
    async fn peek() {
        let ring = Ring::new();
        assert_eq!(ring.peek(lambda).await, None);
        ring.push(1).await;
        assert_eq!(ring.peek(lambda).await, Some(1));
        assert_eq!(ring.poll().await, Some(1));
        assert_eq!(ring.peek(lambda).await, None);
    }
}
