# Interview Questions

## Embedded/Low-Level

1. [Valid Parentheses (Easy)](../leetcode/valid_parentheses_20.rs): start with naive implementation of stack, then move to counter-based solution for low memory footprint (embedded) targets.
2. [Concurrent Ring Buffer (Medium)](./concurrent_ring_buffer.rs): you’re given a bounded, thread-safe ring buffer intended to be used by multiple producers and consumers. It’s implemented with [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html), and [`Condvar`](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html), but it has several concurrency bugs and logic errors. Make the provided code run reliably under contention: no deadlocks, no panics from poisoned mutexes, correct wrap-around behavior, producers block when full / consumers block when empty.
```
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

pub struct BrokenRing<T> {
    // BUG: Two mutexes guard coupled state; the single condvar is used with both.
    // This can panic and/or deadlock. (Condvar must be bound to one mutex.)
    head: Mutex<usize>,
    tail: Mutex<usize>,
    buf: Mutex<Vec<T>>,      // BUG: Stores T directly; no room for "empty" vs "filled"
    cap: usize,
    cv: Condvar,
}

impl<T: Clone + Default> BrokenRing<T> {
    pub fn with_capacity(cap: usize) -> Self {
        // BUG: with_capacity allocates but length is 0; indexing later will panic.
        Self {
            head: Mutex::new(0),
            tail: Mutex::new(0),
            buf: Mutex::new(Vec::with_capacity(cap)),
            cap,
            cv: Condvar::new(),
        }
    }

    pub fn push(&self, value: T) {
        // BUG: Lock order head -> tail -> buf here, different elsewhere -> deadlock risk.
        let mut h = self.head.lock().unwrap();
        let mut t = self.tail.lock().unwrap();
        let mut b = self.buf.lock().unwrap();

        // BUG: Full detection is wrong; also uses == cap instead of tracking len.
        while (*h + 1) % self.cap == *t {
            // BUG: wait without loop might return spuriously; also using tail's guard elsewhere.
            b = self.cv.wait(b).unwrap();
        }

        // BUG: writes past current len (vec len 0) -> panic; and no wrap-around indexing.
        if b.len() < self.cap {
            b.push(value);
        } else {
            b[*h] = value;
        }
        *h = (*h + 1) % self.cap;

        // BUG: notify under the lock set for wrong guard set; and not notifying on pop site.
        self.cv.notify_one();

        drop(t);
        drop(h);
        drop(b);
    }

    pub fn pop(&self) -> T {
        // BUG: Different lock acquisition order (tail -> head -> buf) -> can deadlock.
        let mut t = self.tail.lock().unwrap();
        let mut h = self.head.lock().unwrap();
        let mut b = self.buf.lock().unwrap();

        while *h == *t {
            // BUG: waits on cv with a different mutex than in push -> UB/panic.
            h = self.cv.wait(h).unwrap();
        }

        // BUG: No Option<T>, so we can't represent "empty"; this clones defaults randomly.
        let v = b[*t].clone();
        *t = (*t + 1) % self.cap;

        self.cv.notify_one();
        v
    }
}

// A tiny demo that often deadlocks/panics under load.
pub fn demo() {
    let q = Arc::new(BrokenRing::<usize>::with_capacity(8));
    let producers: Vec<_> = (0..2).map(|id| {
        let q = q.clone();
        thread::spawn(move || {
            for i in 0..10_000 {
                q.push(i + id * 10_000);
            }
        })
    }).collect();

    let consumers: Vec<_> = (0..2).map(|_| {
        let q = q.clone();
        thread::spawn(move || {
            let mut sum = 0usize;
            for _ in 0..10_000 {
                sum = sum.wrapping_add(q.pop());
            }
            sum
        })
    }).collect();

    for p in producers { let _ = p.join(); }
    for c in consumers { let _ = c.join(); }
}
```

