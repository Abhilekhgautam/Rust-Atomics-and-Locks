// Channels can be used to send data between threads.
//
// Mutex Based Channels
//
// Use a VecDeque,
// send pushes  item to it
// receive pops item from it

/*
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        //notify to any one waiting thread
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut val = self.queue.lock().unwrap();
        loop {
            if let Some(v) = val.pop_front() {
                return v;
            }
            //block the thread until you receive a notification
            val = self.item_ready.wait(val).unwrap();
        }
    }
}
*/
// One-Shot Channel: Sending exactly one msg frm one thread to another.
//

use std::cell::UnsafeCell;
use std::mem::MaybeUninit; // unsafe Option<T>
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

impl<T> Channel<T> {
    fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }

    pub fn send(&self, value: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!("Can't send more than one message");
        }

        unsafe { (*self.message.get()).write(value) };
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    pub fn recieve(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("No Message Available");
        }
        // Safety: We already reset the ready flag
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

fn main() {
    let chan = Channel::new();
    let t = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            chan.send("Hello, World");
            t.unpark();
        });

        if !chan.is_ready() {
            thread::park();
        }
        assert_eq!(chan.recieve(), "Hello, World");
    });
}
