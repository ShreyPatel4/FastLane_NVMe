use crate::error::{CoreError, CoreResult};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A lock-free single-producer single-consumer ring buffer backed by atomics.
///
/// The implementation assumes that only one producer pushes to the queue while
/// a single consumer pops from it. This allows the use of simple atomic
/// load/store operations without the need for compare-and-swap loops.
pub struct SpscRing<T> {
    buffer: Vec<UnsafeCell<MaybeUninit<T>>>,
    capacity: usize,
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl<T: Send> Send for SpscRing<T> {}
unsafe impl<T: Send> Sync for SpscRing<T> {}

impl<T> SpscRing<T> {
    /// Create a new ring with the provided capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0, "ring capacity must be non-zero");
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize_with(capacity, || UnsafeCell::new(MaybeUninit::uninit()));
        Self {
            buffer,
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Attempt to push a value onto the ring.
    pub fn push(&self, value: T) -> CoreResult<()> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        if head.wrapping_sub(tail) == self.capacity {
            return Err(CoreError::RingFull);
        }

        let index = head % self.capacity;
        unsafe {
            (*self.buffer[index].get()).write(value);
        }
        self.head.store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Pop a value from the ring if one is available.
    pub fn pop(&self) -> CoreResult<T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail == head {
            return Err(CoreError::RingEmpty);
        }

        let index = tail % self.capacity;
        let value = unsafe { (*self.buffer[index].get()).assume_init_read() };
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Ok(value)
    }

    /// Returns the number of elements currently stored in the ring.
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        head.wrapping_sub(tail)
    }

    /// Whether the ring is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Total capacity of the ring in number of elements.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Drop for SpscRing<T> {
    fn drop(&mut self) {
        let mut tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Relaxed);
        while tail != head {
            let index = tail % self.capacity;
            unsafe {
                (*self.buffer[index].get()).assume_init_drop();
            }
            tail = tail.wrapping_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SpscRing;
    use crate::error::CoreError;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn fifo_order_is_preserved() {
        let ring = SpscRing::with_capacity(4);
        ring.push(1).unwrap();
        ring.push(2).unwrap();
        ring.push(3).unwrap();

        assert_eq!(ring.len(), 3);
        assert_eq!(ring.pop().unwrap(), 1);
        assert_eq!(ring.pop().unwrap(), 2);
        assert_eq!(ring.pop().unwrap(), 3);
        assert!(matches!(ring.pop(), Err(CoreError::RingEmpty)));
        assert!(ring.is_empty());
    }

    #[test]
    fn reports_full_when_capacity_reached() {
        let ring = SpscRing::with_capacity(2);
        ring.push(10).unwrap();
        ring.push(20).unwrap();
        assert!(matches!(ring.push(30), Err(CoreError::RingFull)));
    }

    #[test]
    fn supports_concurrent_single_producer_consumer() {
        let ring = Arc::new(SpscRing::with_capacity(32));
        let producer = ring.clone();
        let consumer = ring.clone();

        let producer_thread = thread::spawn(move || {
            for value in 0..1_000 {
                loop {
                    if producer.push(value).is_ok() {
                        break;
                    }
                    thread::yield_now();
                }
            }
        });

        let consumer_thread = thread::spawn(move || {
            let mut next = 0;
            while next < 1_000 {
                match consumer.pop() {
                    Ok(value) => {
                        assert_eq!(value, next);
                        next += 1;
                    }
                    Err(CoreError::RingEmpty) => {
                        thread::sleep(Duration::from_micros(10));
                    }
                    Err(other) => panic!("unexpected error: {other:?}"),
                }
            }
        });

        producer_thread.join().unwrap();
        consumer_thread.join().unwrap();
        assert!(ring.is_empty());
    }
}
