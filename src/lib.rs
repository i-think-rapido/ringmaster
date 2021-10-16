#![allow(dead_code)]
use crate::Slot::{Root, Box};
use crate::Mode::{FIFO, LIFO};
use tokio::sync::{Mutex, MutexGuard};

enum Slot {
    Root{ prev: usize, next: usize },
    Box{ prev: usize, next: usize, buffer_idx: usize },
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
    buffer: Mutex<Vec<T>>,
    linked_list: Mutex<Vec<Slot>>,
    mode: Mode,
}

impl<T> Ring<T> {

    fn new() -> Self {
        Self {
            buffer: Mutex::new(vec![]),
            linked_list: Mutex::new(vec![Root{prev: 0, next: 0}]),
            mode: FIFO,
        }
    }

    fn as_fifo(self) -> Self {
        Self {
            buffer: self.buffer,
            linked_list: self.linked_list,
            mode: FIFO,
        }
    }

    fn as_lifo(self) -> Self {
        Self {
            buffer: self.buffer,
            linked_list: self.linked_list,
            mode: LIFO,
        }
    }

    async fn from(vec: Vec<T>) -> Self {

        let mut count = 1;
        let mut list = vec![Root{prev: count, next: vec.len() as usize }];
        for _ in 0..vec.len() {
            list.push(Box{buffer_idx: count - 1, next: count - 1, prev: count + 1 });
            count += 1;
        }
        if let Some(Box{prev, ..}) = list.get_mut(count - 1) {
            *prev = 0;
        }

        Self {
            buffer: Mutex::new(vec),
            linked_list: Mutex::new(list),
            mode: FIFO,
        }
    }

    async fn is_empty(&self) -> bool {
        self.linked_list.lock().await.len() == 1
    }

    async fn len(&self) -> usize {
        self.buffer.lock().await.len()
    }

    async fn push(&self, item: T) {
        let mut vec = self.buffer.lock().await;
        let mut list = self.linked_list.lock().await;
        match list.len() {
            0 => unreachable!(),
            1 => {
                vec.push(item);

                list.clear();
                list.push(Root{prev:1, next:1});
                list.push(Box{prev:0, next:0, buffer_idx: 0});
            },
            len => {
                if let Root{next, ..} = list[0] {
                    vec.push(item);

                    list.push(Box{prev:0, next, buffer_idx: len - 1});
                    if let Some(Box{prev, ..}) = list.get_mut(next) {
                        *prev = len;
                    }
                    if let Some(Root{next, ..}) = list.get_mut(0) {
                        *next = len;
                    }
                }
            }
        }
    }

    async fn swap_remove(&self, 
        pos: usize, 
        vec: &mut MutexGuard<'_, Vec<T>>,
        list: &mut MutexGuard<'_, Vec<Slot>>
    ) -> Option<T> {
        let len = list.len();
        match len {
            0 => unreachable!(),
            1 => (),
            2 => {
                if let Box{buffer_idx, ..} = list.remove(1) {
                    list.clear();
                    list.push(Root{prev:0, next:0});
                    return Some(vec.remove(buffer_idx))
                }
            }
            _ => {
                if let Box { buffer_idx, prev: bprev, next: bnext } = list.swap_remove(pos) {
                    match list.get_mut(bnext) {
                        Some(Root { prev: p, .. }) => {
                            *p = bprev;
                        }
                        Some(Box { prev: p, .. }) => {
                            *p = bprev;
                        }
                        _ => ()
                    }
                    match list.get_mut(bprev) {
                        Some(Root { next: n, .. }) => {
                            *n = bnext;
                        }
                        Some(Box { next: n, .. }) => {
                            *n = bnext;
                        }
                        _ => unreachable!()
                    }
                    if let Some(&Box { prev: lprev, next: lnext, .. }) = list.get(pos) {
                        match list.get_mut(lprev) {
                            Some(Root { next: n, .. }) => {
                                *n = pos;
                            }
                            Some(Box { next: n, .. }) => {
                                *n = pos;
                            }
                            _ => unreachable!()
                        }
                        match list.get_mut(lnext) {
                            Some(Root { prev: p, .. }) => {
                                *p = pos;
                            }
                            Some(Box { prev: p, .. }) => {
                                *p = pos;
                            }
                            _ => unreachable!()
                        }
                    }
                    if let Some(Box{ buffer_idx: idx, .. }) = list.get_mut(pos) {
                        *idx = buffer_idx;
                    }
                    return Some(vec.swap_remove(buffer_idx))
                }
            }
        }
        None
    }

    async fn poll(&self) -> Option<T> {
        let mut vec = self.buffer.lock().await;
        let mut list = self.linked_list.lock().await;
        if let Root { prev, next } = list[0] {
            let pos = match self.mode {
                FIFO => prev,
                LIFO => next,
            };
            return self.swap_remove(pos, &mut vec, &mut list).await;
        }
        None
    }

    async fn poll_with(&self, f: fn(&T) -> bool) -> Option<T> {
        let vec = self.buffer.lock().await;
        let list = self.linked_list.lock().await;
        if let Root{prev, next} = list[0] {

            let pos = match self.mode {
                FIFO => prev,
                LIFO => next,
            };
            if let Some(Box { buffer_idx, .. }) = list.get(pos) {
                if let Some(item) = vec.get(*buffer_idx) {
                    if f(item) {
                        drop(list);
                        drop(vec);
                        return self.poll().await
                    }
                }
            }
        }
        None
    }

    async fn peek<R>(&self, f: fn(&T) -> Option<R>) -> Option<R> {
        let vec = self.buffer.lock().await;
        let list = self.linked_list.lock().await;
        let v = &*list;
        if let Root{prev, next} = v[0] {

            let pos = match self.mode {
                FIFO => prev,
                LIFO => next,
            };
            if let Some(Box { buffer_idx, .. }) = v.get(pos) {
                if let Some(item) = vec.get(*buffer_idx) {
                    return f(item)
                }
            }
        }
        None
    }

    async fn fold_fast<R>(&self, start: R, f: fn(R, &T) -> R) -> R {
        let mut acc = start;

        let buffer = self.buffer.lock().await;
        for item in buffer.iter() {
            acc = f(acc, item);
        }

        acc
    }

    async fn fold<R>(&self, start: R, f: fn(R, &T) -> R) -> R {
        let mut acc = start;

        let vec = self.buffer.lock().await;
        let list = self.linked_list.lock().await;
        let mut current = &list[0];
        let mut root_already_visited = false;
        match current {
            Root{prev, ..} => if *prev == 0 { return acc },
            _ => unreachable!(),
        }

        loop {
            match current {
                Root{prev, ..} => {
                    if root_already_visited {
                        return acc
                    }
                    else {
                        current = list.get(*prev).unwrap();
                        root_already_visited = true
                    }
                },
                Box{ buffer_idx, prev, ..} => {
                    acc = f(acc, vec.get(*buffer_idx).unwrap());
                    current = list.get(*prev).unwrap();
                }
            } 
        }
    }

    async fn purge(&self, f: fn(&T) -> bool) {
    
        let mut vec = self.buffer.lock().await;
        let mut list = self.linked_list.lock().await;

        let mut pos = 0;
        while pos < vec.len() {
            if let Some(item) = vec.get(pos) {
                if !f(item) {
                    let _ = self.swap_remove(pos + 1, &mut vec, &mut list).await;
                    continue;
                }
            }
            else {
                break;
            }
            pos += 1;
        }
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
    async fn length() {
        let ring = Ring::from(vec![1,2,3,4,5]).await;
        assert_eq!(ring.len().await, 5);
    }

    #[tokio::test]
    async fn push() {
        let ring = Ring::new();
        ring.push(1).await;
        let vec = ring.buffer.lock().await;
        let list = ring.linked_list.lock().await;
        if let Root{prev, next} = list[0] {
            assert_eq!(prev, 1);
            assert_eq!(next, 1);
        }
        if let Some(&Box{prev, next, buffer_idx}) = list.last() {
            assert_eq!(prev, 0);
            assert_eq!(next, 0);
            assert_eq!(vec.get(buffer_idx), Some(&1));
        }
    }

    #[tokio::test]
    async fn from() {
        let ring = Ring::from(vec![1,3,4,6]).await;
        let vec = ring.buffer.lock().await;
        let list = ring.linked_list.lock().await;
        if let Some(&Box{prev, next, buffer_idx}) = list.last() {
            assert_eq!(prev, 0);
            assert_eq!(next, 3);
            assert_eq!(vec.get(buffer_idx), Some(&6));
        }
        if let Box{prev, next, buffer_idx} = list[1] {
            assert_eq!(prev, 2);
            assert_eq!(next, 0);
            assert_eq!(vec.get(buffer_idx), Some(&1));
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

    #[tokio::test]
    async fn fold_fast() {
        let ring = Ring::from(vec![1,2,3,4,5]).await;
        let result = ring.fold_fast(0, |acc, item| acc + item).await;
        assert_eq!(result, 15);

        assert_eq!(ring.poll().await, Some(1));
        ring.push(6).await;
        let result = ring.fold(0, |acc, item| acc + item).await;
        assert_eq!(result, 20);
    }

    #[tokio::test]
    async fn fold() {
        let ring = Ring::from(vec![1,2,3,4,5]).await;
        let result = ring.fold(0, |acc, item| acc + item).await;
        assert_eq!(result, 15);

        assert_eq!(ring.poll().await, Some(1));
        ring.push(6).await;
        let result = ring.fold(0, |acc, item| acc + item).await;
        assert_eq!(result, 20);
    }

    #[tokio::test]
    async fn purge() {
        let ring = Ring::from(vec![1,2,3,4,5]).await;
        ring.purge(|x| { x % 2 == 0 }).await;
        assert_eq!(ring.len().await, 2);
        assert_eq!(ring.poll().await, Some(2));
        assert_eq!(ring.poll().await, Some(4));
        assert_eq!(ring.poll().await, None);
    }
}
