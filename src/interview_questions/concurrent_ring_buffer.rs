use std::sync::{Condvar, Mutex, PoisonError};

#[derive(Debug)]
struct Inner<T> {
    buf: Vec<Option<T>>,
    cap: usize,
    head: usize, // next write position
    tail: usize, // next read position
    len: usize,  // number of elements stored
}

impl<T> Inner<T> {
    fn new(cap: usize) -> Self {
        Self {
            buf: (0..cap).map(|_| None).collect(),
            cap,
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.len == self.cap
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn inc_wrap(idx: usize, cap: usize) -> usize {
        let n = idx + 1;
        if n == cap {
            0
        } else {
            n
        }
    }
}

pub struct RingBuffer<T> {
    inner: Mutex<Inner<T>>,
    not_empty: Condvar,
    not_full: Condvar,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "capacity must be > 0");
        Self {
            inner: Mutex::new(Inner::new(cap)),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }

    /// Pushes an item, blocking if the buffer is full
    pub fn push(&self, val: T) {
        // Handle poisoning by continuing (we own the guard via into_inner()).
        let mut g = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(), // chosen policy: recover and continue
        };

        while g.is_full() {
            // Wait releases the mutex and reacquires it before returning. Always wait in a loop to
            // re-check the predicate
            g = self.not_full.wait(g).unwrap();
        }

        let idx = g.head;
        debug_assert!(g.buf[idx].is_none());
        g.buf[idx] = Some(val);
        g.head = Inner::<T>::inc_wrap(g.head, g.cap);
        g.len += 1;

        // Notify one consumer that an item is now available
        self.not_empty.notify_one();
        // Mutex guard drops here...
    }

    /// Pops an item, blocking if the buffer is empty
    pub fn pop(&self) -> T {
        let mut g = match self.inner.lock() {
            Ok(guard) => guard,
            Err(PoisonError { .. }) => {
                // Same policy as push: recover
                // Safety: contents may be inconsistent if producer panicked mid-update,
                // but our state transitions keep invariants, so we proceed.
                // In a different policy, you could propagate an error instead.
                // (see Mutex poisoning docs)
                // :contentReference[oaicite:10]{index=10}
                // Acquire again and recover.
                self.inner.lock().unwrap_or_else(|p| p.into_inner())
            }
        };

        while g.is_empty() {
            g = self.not_empty.wait(g).unwrap();
        }

        let idx = g.tail;
        let val = g.buf[idx].take().expect("invariant: Some on pop");
        g.tail = Inner::<T>::inc_wrap(g.tail, g.cap);
        g.len -= 1;

        // Notify one producer that there is free space
        self.not_full.notify_one();
        val
    }

    /// Non-blocking push variant (useful for tests)
    pub fn try_push(&self, val: T) -> Result<(), T> {
        let mut g = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        if g.is_full() {
            return Err(val);
        }
        let idx = g.head;
        g.buf[idx] = Some(val);
        g.head = Inner::<T>::inc_wrap(g.head, g.cap);
        g.len += 1;
        self.not_empty.notify_one();
        Ok(())
    }

    /// Non-blocking pop variant
    pub fn try_pop(&self) -> Option<T> {
        let mut g = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        if g.is_empty() {
            return None;
        }
        let idx = g.tail;
        let val = g.buf[idx].take();
        g.tail = Inner::<T>::inc_wrap(g.tail, g.cap);
        g.len -= 1;
        self.not_full.notify_one();
        val
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.inner.lock().unwrap().len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn simple_thread_basic() {
        let q = RingBuffer::with_capacity(3);
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.try_push(4), Err(4));
        assert_eq!(q.pop(), 1);
        q.push(4);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.pop(), 3);
        assert_eq!(q.pop(), 4);
        assert!(q.try_pop().is_none());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn multi_producer_consumer() {
        let q = Arc::new(RingBuffer::with_capacity(64));
        let produced = Arc::new(AtomicUsize::new(0));
        let consumed = Arc::new(AtomicUsize::new(0));

        // 3 producers
        let mut threads = vec![];
        for _ in 0..3 {
            let q = q.clone();
            let p = produced.clone();
            threads.push(std::thread::spawn(move || {
                for _ in 0..10_000 {
                    q.push(1u8);
                    p.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }

        // 3 consumers
        for _ in 0..3 {
            let q = q.clone();
            let c = consumed.clone();
            threads.push(std::thread::spawn(move || {
                for _ in 0..10_000 {
                    let _ = q.pop();
                    c.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }

        for t in threads {
            t.join().unwrap();
        }

        assert_eq!(produced.load(Ordering::Relaxed), 30_000);
        assert_eq!(consumed.load(Ordering::Relaxed), 30_000);
        assert_eq!(q.len(), 0);
    }
}
