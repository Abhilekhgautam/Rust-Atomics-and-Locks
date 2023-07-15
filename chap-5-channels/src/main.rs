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
use std::sync::atomic::Ordering::Release;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub unsafe fn send(&self, value: T) {
        let v = (*self.message.get()).write(value);
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        todo!();
    }
}

fn main() {}
