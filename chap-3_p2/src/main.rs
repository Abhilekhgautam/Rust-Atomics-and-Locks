// Example: Locking

//Mutex are most common use case for release and acquire ordering

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;
static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    // compare_exchange takes 2 memory ordering param
    // 1. for when comparision succeeded
    // 2. for when camparision failed.
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        //Safety: We hold the lock, so nothing else is accessing DATA
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    })
}
