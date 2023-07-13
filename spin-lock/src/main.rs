// Spin Lock

// Spin Lock is a lock that causes a thread trying to acquire it to simply
// wait in a loop while continuously checking whether the lock is available.

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

use std::cell::UnsafeCell;

use std::ops::{Deref, DerefMut};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// UnsafeCell doesn't implement Sync, so our type is no longer
// shareable between threads so we need to imiplement Sync for our SpinLock

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            // tell the processor that we're spinning while waiting for sth to change.
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: The existence of this Guard guarantess we've exclusively
        // locked the lock
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: The existence of this guard guarantess we've exclusively
        // locked the lock

        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

use std::thread;
fn main() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
